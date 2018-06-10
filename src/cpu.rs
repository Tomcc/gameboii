extern crate std;

use bit_field::BitField;
use interpreter;
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use std::time::Instant;

pub union Register {
    pub r8: (u8, u8),
    pub r16: u16,
}

//the RAM size is max addr + 1
const RAM_SIZE: usize = 0xFFFF + 1;
const MACHINE_HZ: u64 = 4194304;

#[allow(unused)]
const HORIZONTAL_SYNC_HZ: u64 = 9198000;
#[allow(unused)]
const VERTICAL_SYNC_HZ: f32 = 59.73;

#[allow(unused)]
const VERTICAL_BLANK_INTERRUPT_START_ADDRESS: u16 = 0x40;
#[allow(unused)]
const LCDC_STATUS_INTERRUPT_START_ADDRESS: u16 = 0x48;
#[allow(unused)]
const SERIAL_TRANSFER_COMPLETION_INTERRUPT_START_ADDRESS: u16 = 0x58;
#[allow(unused)]
const HIGH_TO_LOW_P10_P13_INTERRUPT_START_ADDRESS: u16 = 0x60;
#[allow(unused)]
const COLOR_GB_ENABLE_ADDRESS: u16 = 0x143;
#[allow(unused)]
const SUPER_GB_ENABLE_ADDRESS: u16 = 0x146;
const CARTRIDGE_TYPE_ADDRESS: u16 = 0x147;
#[allow(unused)]
const ROM_SIZE_ADDRESS: u16 = 0x148;
#[allow(unused)]
const RAM_SIZE_ADDRESS: u16 = 0x149;
const INTERNAL_ROM_TURN_OFF_ADDRESS: u16 = 0xFF50;

//derived data
const TICK_DURATION: Duration = Duration::from_nanos(1000000000 / MACHINE_HZ);

//TODO move to CPU/screen
#[allow(unused)]
const RESOLUTION_W: u32 = 160;
#[allow(unused)]
const RESOLUTION_H: u32 = 144;
#[allow(unused)]
const MAX_SPRITES: u32 = 40;
#[allow(unused)]
const MAX_SPRITES_PER_LINE: u32 = 10;
#[allow(unused)]
const MAX_SPRITE_SIZE_W: u32 = 8;
#[allow(unused)]
const MAX_SPRITE_SIZE_H: u32 = 16;
#[allow(unused)]
const MIN_SPRITE_SIZE_W: u32 = 8;
#[allow(unused)]
const MIN_SPRITE_SIZE_H: u32 = 8;

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
}

impl<'a> CPU<'a> {
    pub fn new(rom: &'a [u8]) -> CPU<'a> {
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
        };

        //setup stuff
        assert!(
            rom[CARTRIDGE_TYPE_ADDRESS as usize] == 0,
            "Not a ROM-Only ROM, not supported"
        );

        // try to load the "bios" rom into the start of the buffer
        File::open("DMG_ROM.bin")
            .unwrap()
            .read_exact(&mut cpu.RAM[0..100])
            .unwrap();

        cpu
    }

    pub fn run(&mut self) {
        loop {
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
        for _ in 0..count {
            //TODO do the thing

            //then spin until the next clock.
            //these clocks are too short to sleep, which has a ~1ms precision or worse.
            //so we have to spin and hope that the OS doesn't hate us
            //TODO going as fast as possible by disabling this
            self.next_clock_time += TICK_DURATION;
            while Instant::now() < self.next_clock_time {
                std::thread::yield_now();
            }
        }
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
        if addr == INTERNAL_ROM_TURN_OFF_ADDRESS && val == 1{
            //load the cartridge rom
            self.RAM.copy_from_slice(self.cartridge_ROM);
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

    pub unsafe fn set_z(&mut self, val: bool) {
        self.AF.r8.1.set_bit(7, val);
    }
    pub unsafe fn set_n(&mut self, val: bool) {
        self.AF.r8.1.set_bit(6, val);
    }
    pub unsafe fn set_h(&mut self, val: bool) {
        self.AF.r8.1.set_bit(5, val);
    }
    pub unsafe fn set_c(&mut self, val: bool) {
        self.AF.r8.1.set_bit(4, val);
    }

    pub unsafe fn z(&self) -> bool {
        self.AF.r8.1.get_bit(7)
    }
    pub unsafe fn c(&self) -> bool {
        self.AF.r8.1.get_bit(4)
    }
}
