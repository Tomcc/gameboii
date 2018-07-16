extern crate libgameboii;

use libgameboii::cpu::CPU;
use libgameboii::ppu::PPU;
use std::path::Path;
use std::str;

#[derive(Debug, PartialEq, Clone, Copy)]
enum TestState {
    Running,
    Failed,
    Passed,
}

struct TestOut {
    state: TestState,
    buffer: String,
}

impl TestOut {
    fn new() -> Self {
        TestOut {
            state: TestState::Running,
            buffer: String::new(),
        }
    }
}

impl std::io::Write for TestOut {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // check if the test should stop
        self.buffer += str::from_utf8(buf).unwrap();

        if self.buffer.ends_with("Passed") {
            self.state = TestState::Passed;
        } else if self.buffer.ends_with("Failed") {
            self.state = TestState::Failed;
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn run_test(path: &Path) {
    println!("{:?}", std::env::current_dir().unwrap());
    let rom = libgameboii::open_rom(&path).unwrap();
    // TODO load from a savestate instead
    let bootrom = libgameboii::open_rom(&"../ROMs/DMG_ROM.bin").unwrap();

    let mut ppu = PPU::new();
    let mut cpu = CPU::new(&rom, &bootrom);

    let mut current_clock = 0;

    let mut serial_out = TestOut::new();
    {
        let mut update = |cpu: &mut CPU, ppu: &mut PPU| {
            cpu.tick(current_clock, &mut None, &mut serial_out);
            ppu.tick(cpu, current_clock);

            current_clock += 1;

            cpu.should_exit == false && serial_out.state == TestState::Running
        };

        while update(&mut cpu, &mut ppu) {}
    }
    println!("{}", serial_out.buffer);

    assert_eq!(serial_out.state, TestState::Passed);
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
