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
const BOOT_ROM_SIZE: usize = 0x100;

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

    boot_mode: bool,
    cb_mode: bool,

    interrupt_change_counter: u8,
    interrupts_master_enabled_next: u8,
    interrupts_master_enabled: u8,
    requested_interrupts: u8,

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

            boot_mode: true,

            interrupts_master_enabled: 0,
            interrupt_change_counter: 0,
            interrupts_master_enabled_next: 0,
            requested_interrupts: 0,

            next_clock_time: Instant::now(),
            cartridge_ROM: rom,
            log: if do_log { Some(Log::new()) } else { None },
        };

        //copy the ROM in memory
        cpu.RAM[0..rom.len()].copy_from_slice(rom);

        //setup stuff
        assert!(
            rom[address::CARTRIDGE_TYPE] == 0,
            "Not a ROM-Only ROM, not supported"
        );

        // override the first 256 bytes with the Nintendo boot ROM
        File::open("ROMs/DMG_ROM.bin")
            .unwrap()
            .read_exact(&mut cpu.RAM[0..BOOT_ROM_SIZE])
            .unwrap();

        cpu
    }

    fn find_highest_prio_interrupt(&self) -> usize {
        for i in 0..5 {
            if self.requested_interrupts.get_bit(i) {
                return i as usize;
            }
        }
        panic!("Only call this if any interrupt is requested");
    }

    pub fn handle_interrupts(&mut self) -> bool {
        // handle interrupts:
        // if any bit of interrupts_requested are set and enabled, start from the
        // highest priority (0) and switch to the interrupt routine
        let interrupts = self.interrupts_master_enabled
            & self.requested_interrupts
            & self.RAM[address::IE_REGISTER];

        if interrupts != 0 {
            // interrupts are available: find the highest priority one
            let current_interrupt = self.find_highest_prio_interrupt();

            // disable the IME
            self.enable_interrupts(false);
            // reset the bit we handled
            self.requested_interrupts.set_bit(current_interrupt, false);

            // call the new address
            let addr = address::INTERRUPT[current_interrupt] as u16;
            self.call(addr);

            // then wait 5 cycles (according to the wiki?)
            self.run_cycles(5);

            return true;
        }
        return false;
    }

    pub fn tick(&mut self) -> bool {
        if Instant::now() > self.next_clock_time {
            let has_log = self.log.is_some();
            if has_log {
                let pc = self.PC;
                let instr = self.peek_instruction();
                if let Some(ref mut log) = self.log {
                    log.log_instruction(instr, pc).unwrap();
                }
            }

            if self.handle_interrupts() {
                //skip the rest of the "instruction" because this isn't a real instruction
                return true;
            }

            unsafe {
                if self.cb_mode {
                    interpreter::interpret_cb(self);
                    self.cb_mode = false;
                } else {
                    interpreter::interpret(self);
                }
            }

            if self.interrupt_change_counter > 0 {
                self.interrupt_change_counter -= 1;
                if self.interrupt_change_counter == 0 {
                    self.interrupts_master_enabled = self.interrupts_master_enabled_next;
                    assert!(
                        self.interrupts_master_enabled == 0,
                        "Interrupts not supported"
                    );
                }
            }

            return true;
        }
        false
    }

    pub fn enable_cb(&mut self) {
        self.cb_mode = true;
    }

    pub fn enable_interrupts(&mut self, future_state: bool) {
        self.interrupt_change_counter = 2;
        self.interrupts_master_enabled_next = match future_state {
            true => u8::max_value(),
            false => 0,
        };
    }

    pub fn change_interrupt_flags(&mut self, new_value: u8) {
        let old = self.RAM[address::IF_REGISTER];
        let changed = (!old) & new_value;
        self.requested_interrupts |= changed;
    }

    pub fn request_vblank(&mut self) {
        let mut new_request = self.requested_interrupts;
        new_request.set_bit(0, true);
        self.change_interrupt_flags(new_request);
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
        self.RAM[addr as usize]
    }

    pub fn address16(&self, addr: u16) -> u16 {
        let b1 = self.address(addr) as u16;
        let b2 = self.address(addr + 1) as u16;

        (b2 << 8) | b1
    }

    pub fn set_address(&mut self, addr: u16, val: u8) {
        let addr = addr as usize;
        //TODO how to not check this for every set ever?
        if self.boot_mode && addr == address::INTERNAL_ROM_TURN_OFF {
            //replace the Nintendo boot ROM with the first 256 bytes of the cart
            self.RAM[0..BOOT_ROM_SIZE].copy_from_slice(&self.cartridge_ROM[0..BOOT_ROM_SIZE]);
            self.boot_mode = false;
        } else if addr == address::IF_REGISTER {
            self.change_interrupt_flags(val);
        } else if val != 0 {
            address::check_unimplemented(addr);
        }
        self.RAM[addr] = val;
    }

    pub fn set_address16(&mut self, addr: u16, val: u16) {
        let addr = addr as usize;
        address::check_unimplemented(addr);

        self.RAM[addr] = val as u8;
        self.RAM[addr + 1] = (val >> 8) as u8;
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

    pub fn call(&mut self, addr: u16) {
        let pc = self.PC;
        self.push16(pc);
        self.PC = addr;
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
