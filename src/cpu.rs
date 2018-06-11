extern crate std;

use address;
use bit_field::BitField;
use debug_log::Log;
use interpreter;
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use std::time::Instant;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct RegisterPair {
    pub second: u8,
    pub first: u8,
}

#[repr(C)]
pub union Register {
    pub r8: RegisterPair,
    pub r16: u16,
}

//the RAM size is max addr + 1
const RAM_SIZE: usize = 0xFFFF + 1;
pub const MACHINE_HZ: u64 = 4194304;

//derived data
const TICK_DURATION: Duration = Duration::from_nanos(1000000000 / MACHINE_HZ);

#[allow(unused)]
const SOUND_CHANNELS: u32 = 4;

#[allow(non_snake_case)]
pub struct CPU<'a> {
    pub PC: u16,
    pub SP: u16,
    pub AF: Register,
    pub BC: Register,
    pub DE: Register,
    pub HL: Register,

    pub RAM: [u8; RAM_SIZE],

    pub cb_mode: bool,

    next_clock_time: Instant,
    cartridge_ROM: &'a [u8],
    log: Option<Log>,
}

impl<'a> CPU<'a> {
    pub fn new(rom: &'a [u8], do_log: bool) -> CPU<'a> {
        let mut cpu = CPU {
            PC: 0,
            SP: 0,
            AF: Register { r16: 0 },
            BC: Register { r16: 0 },
            DE: Register { r16: 0 },
            HL: Register { r16: 0 },
            RAM: [0; RAM_SIZE],
            cb_mode: false,

            next_clock_time: Instant::now(),
            cartridge_ROM: rom,
            log: if do_log { Some(Log::new()) } else { None },
        };

        //copy the ROM in memory
        cpu.RAM[0..rom.len()].copy_from_slice(rom);

        //setup stuff
        assert!(
            rom[address::CARTRIDGE_TYPE as usize] == 0,
            "Not a ROM-Only ROM, not supported"
        );

        // override the first 256 bytes with the Nintendo boot ROM
        File::open("DMG_ROM.bin")
            .unwrap()
            .read_exact(&mut cpu.RAM[0..0x100])
            .unwrap();

        cpu
    }

    pub fn tick(&mut self) {
        if Instant::now() > self.next_clock_time {
            let has_log = self.log.is_some();
            if has_log {
                let pc = self.PC;
                let instr = self.peek_instruction();
                if let Some(ref mut log) = self.log {
                    log.log_instruction(instr, pc).unwrap();
                }
            }

            unsafe {
                if self.cb_mode {
                    interpreter::interpret_cb(self);
                    self.cb_mode = false;
                } else {
                    interpreter::interpret(self);
                }
            }
        }
    }

    pub fn peek_instruction(&self) -> u8 {
        self.address(self.PC)
    }

    pub fn immediate_u16(&self) -> u16 {
        //assuming that the PC is at the start of the instruction
        self.address16(self.PC + 1)
    }
    pub fn immediate_u8(&self) -> u8 {
        //assuming that the PC is at the start of the instruction
        self.address(self.PC + 1)
    }
    pub fn immediate_i8(&self) -> i8 {
        //assuming that the PC is at the start of the instruction
        unsafe { std::mem::transmute::<u8, i8>(self.address(self.PC + 1)) }
    }

    pub fn run_cycles(&mut self, count: usize) {
        self.next_clock_time += TICK_DURATION * (count as u32);
    }

    pub fn address(&self, addr: u16) -> u8 {
        //          Interrupt Enable Register
        //          --------------------------- FFFF
        //          Internal RAM
        //          --------------------------- FF80
        //          Empty but unusable for I/O
        //          --------------------------- FF4C
        //          I/O ports
        //          --------------------------- FF00
        //          Empty but unusable for I/O
        //          --------------------------- FEA0
        //          Sprite Attrib Memory (OAM)
        //          --------------------------- FE00
        //          Echo of 8kB Internal RAM
        //          --------------------------- E000
        //          8kB Internal RAM
        //          --------------------------- C000
        //          8kB switchable RAM bank
        //          --------------------------- A000
        //          8kB Video RAM
        //          --------------------------- 8000 -
        //          16kB switchable ROM bank         |
        //          --------------------------- 4000 |= 32kB Cartrige
        //          16kB ROM bank #0                 |
        //          --------------------------- 0000 -

        //    * NOTE: b = bit, B = byte

        //TODO the address space and interrupts are a lot more complex than that...
        self.RAM[addr as usize]
    }

    pub fn address16(&self, addr: u16) -> u16 {
        let b1 = self.address(addr) as u16;
        let b2 = self.address(addr + 1) as u16;

        (b2 << 8) | b1
    }

    pub fn set_address(&mut self, addr: u16, val: u8) {
        //TODO how to not check this for every set ever?
        if addr == address::INTERNAL_ROM_TURN_OFF && val == 1 {
            //replace the Nintendo boot ROM with the first 256 bytes of the cart
            self.RAM[0..0x100].copy_from_slice(&self.cartridge_ROM[0..0x100]);
        }

        self.RAM[addr as usize] = val;
    }

    pub fn set_address16(&mut self, addr: u16, val: u16) {
        self.RAM[addr as usize] = val as u8;
        self.RAM[addr as usize + 1] = (val >> 8) as u8;
    }

    pub fn offset_sp(&self, _off: i8) -> u16 {
        panic!("not implemented");
    }

    pub fn push16(&mut self, val: u16) {
        let sp = self.SP;
        self.set_address16(sp, val);
        self.SP = self.SP.wrapping_sub(2);
    }

    pub fn pop16(&mut self) -> u16 {
        self.SP = self.SP.wrapping_add(2);
        let sp = self.SP;
        self.address16(sp)
    }

    pub unsafe fn set_z(&mut self, val: bool) {
        self.AF.r8.second.set_bit(7, val);
    }
    pub unsafe fn set_n(&mut self, val: bool) {
        self.AF.r8.second.set_bit(6, val);
    }
    pub unsafe fn set_h(&mut self, val: bool) {
        self.AF.r8.second.set_bit(5, val);
    }
    pub unsafe fn set_c(&mut self, val: bool) {
        self.AF.r8.second.set_bit(4, val);
    }

    pub unsafe fn z(&self) -> bool {
        self.AF.r8.second.get_bit(7)
    }
    pub unsafe fn c(&self) -> bool {
        self.AF.r8.second.get_bit(4)
    }
}
