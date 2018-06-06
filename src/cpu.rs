extern crate std;

use std::fs::File;
use std::io::Read;
use interpreter;

pub union Register {
    pub r8: (u8, u8),
    pub r16: u16,
}

//the RAM size is max addr + 1
const RAM_SIZE: usize = 0xFFFF + 1;

pub struct CPU {
    pub PC : u16,
    pub SP : u16,
    pub AF: Register,
    pub BC: Register,
    pub DE: Register,
    pub HL: Register,

    pub RAM: [u8; RAM_SIZE],

    pub cb_mode: bool,
}

impl CPU {
    pub fn new() -> Self {
        let mut cpu = CPU {
            PC: 0,
            SP: 0,
            AF: Register { r16: 0 },
            BC: Register { r16: 0 },
            DE: Register { r16: 0 },
            HL: Register { r16: 0 },
            RAM: [0; RAM_SIZE],
            cb_mode: false,
        };

        // try to load the "bios" rom into the start of the buffer
        File::open("DMG_ROM.bin").unwrap().read_exact(&mut cpu.RAM[0..100]).unwrap();

        cpu   
    }

    pub fn run(&mut self, ROM: &[u8]) {
        loop {
            unsafe {
                if self.cb_mode {
                    interpreter::interpret_cb(self);
                }
                else {
                    interpreter::interpret(self);
                }
            }
        }
    }

    pub fn peek_instruction(&self) -> u8 {
        self.address(self.PC)
    }

    pub fn immediateU16(&self) -> u16 {
        //assuming that the PC is at the start of the instruction
        self.address16(self.PC + 1)
    }
    pub fn immediateU8(&self) -> u8 {
        //assuming that the PC is at the start of the instruction
        self.address(self.PC + 1)
    }
    pub fn immediateI8(&self) -> i8 {
        //assuming that the PC is at the start of the instruction
        unsafe {
            std::mem::transmute::<u8, i8>(self.address(self.PC + 1))
        }
    }

    pub fn run_cycles(&mut self, count: usize) {
        //TODO
    }

    pub fn address(&self, addr: u16) -> u8 {
        //TODO the address space and interrupts are a lot more complex than that...
        self.RAM[addr as usize]
    }

    pub fn address16(&self, addr: u16) -> u16 {
        let b1 = self.address(addr) as u16;
        let b2 = self.address(addr + 1) as u16;

        (b2 << 8) | b1 
    }

    pub fn offset_sp(&self, off: i8) -> u16 {
        panic!("not implemented");
    }

    pub fn set_n(&mut self) {
        panic!("not implemented");
    }
    pub fn reset_n(&mut self) {
        panic!("not implemented");
    }
    pub fn assign_z(&mut self, state: bool) {
        panic!("not implemented");
    }
    pub fn set_z(&mut self) {
        panic!("not implemented");
    }
    pub fn reset_z(&mut self) {
        panic!("not implemented");
    }
    pub fn z(&self) -> bool{
        panic!("not implemented");
    }
    pub fn c(&self) -> bool{
        panic!("not implemented");
    }
    pub fn set_h(&mut self) {
        panic!("not implemented");
    }
    pub fn reset_h(&mut self) {
        panic!("not implemented");
    }
    pub fn set_c(&mut self) {
        panic!("not implemented");
    }
    pub fn reset_c(&mut self) {
        panic!("not implemented");
    }
    
}