
pub union Register {
    pub r8: (u8, u8),
    pub r16: u16,
}

pub struct CPU {
    pub PC : u16,
    pub SP : u16,
    pub AF: Register,
    pub BC: Register,
    pub DE: Register,
    pub HL: Register,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            PC: 0,
            SP: 0,
            AF: Register { r16: 0 },
            BC: Register { r16: 0 },
            DE: Register { r16: 0 },
            HL: Register { r16: 0 },
        }
    }

    pub fn peek_instruction(&self) -> u8 {
        panic!("not implemented");
    }

    pub fn immediateU16(&self) -> u16 {
        panic!("not implemented");
    }
    pub fn immediateU8(&self) -> u8 {
        panic!("not implemented");
    }
    pub fn immediateI8(&self) -> i8 {
        panic!("not implemented");
    }

    pub fn run_cycles(&mut self, count: usize) {
        panic!("not implemented");
    }

    pub fn address(&self, addr: u16) -> u8 {
        panic!("not implemented");
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