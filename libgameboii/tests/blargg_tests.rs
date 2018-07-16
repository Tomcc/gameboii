extern crate libgameboii;

use libgameboii::cpu::CPU;
use libgameboii::ppu::PPU;
use std::path::Path;

fn run_test(path: &Path) {
    println!("{:?}", std::env::current_dir().unwrap());
    let rom = libgameboii::open_rom(&path).unwrap();
    // TODO load from a savestate instead
    let bootrom = libgameboii::open_rom(&"../ROMs/DMG_ROM.bin").unwrap();

    let mut ppu = PPU::new();
    let mut cpu = CPU::new(&rom, &bootrom);

    let mut current_clock = 0;

    let mut update = |cpu: &mut CPU, ppu: &mut PPU| {
        cpu.tick(current_clock, &mut None);
        ppu.tick(cpu, current_clock);

        current_clock += 1;

        !cpu.should_exit
    };
}

#[test]
fn cpu_instrs_01() {
    run_test(Path::new(
        "tests/blargg/cpu_instrs/individual/01-special.gb",
    ));
}

#[test]
fn cpu_instrs_02() {
    run_test(Path::new(
        "tests/blargg/cpu_instrs/individual/02-interrupts.gb",
    ));
}

#[test]
fn cpu_instrs_03() {
    run_test(Path::new(
        "tests/blargg/cpu_instrs/individual/03-op sp,hl.gb",
    ));
}

#[test]
fn cpu_instrs_04() {
    run_test(Path::new(
        "tests/blargg/cpu_instrs/individual/04-op r,imm.gb",
    ));
}

#[test]
fn cpu_instrs_05() {
    run_test(Path::new("tests/blargg/cpu_instrs/individual/05-op rp.gb"));
}

#[test]
fn cpu_instrs_06() {
    run_test(Path::new("tests/blargg/cpu_instrs/individual/06-ld r,r.gb"));
}

#[test]
fn cpu_instrs_07() {
    run_test(Path::new(
        "tests/blargg/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb",
    ));
}

#[test]
fn cpu_instrs_08() {
    run_test(Path::new(
        "tests/blargg/cpu_instrs/individual/08-misc instrs.gb",
    ));
}

#[test]
fn cpu_instrs_09() {
    run_test(Path::new("tests/blargg/cpu_instrs/individual/09-op r,r.gb"));
}

#[test]
fn cpu_instrs_10() {
    run_test(Path::new(
        "tests/blargg/cpu_instrs/individual/10-bit ops.gb",
    ));
}

#[test]
fn cpu_instrs_11() {
    run_test(Path::new(
        "tests/blargg/cpu_instrs/individual/11-op a,(hl).gb",
    ));
}
