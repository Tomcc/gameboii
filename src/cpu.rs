
pub struct CPU {
    pub PC : u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            PC: 0
        }
    }

    pub fn NOP() {

    }

    pub fn immediateU16(&self) -> u16 {
        panic!("not implemented");
    }
    pub fn immediateU8(&self) -> u8 {
        panic!("not implemented");
    }

    pub fn immediateU16(&self) -> u16 {
        panic!("not implemented");
    }
}