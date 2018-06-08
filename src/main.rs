extern crate bit_field;

mod cpu;
mod interpreter;
mod function_stubs;

use cpu::CPU;

fn main() {
    let mut cpu = CPU::new();

    let rom = &[0 as u8; 1];
    cpu.run(rom);
}
