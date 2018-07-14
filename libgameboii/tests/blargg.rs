extern crate libgameboii;

use libgameboii::cpu::CPU;
use libgameboii::ppu::PPU;

#[test]
fn cpu_instrs_06() {
    println!("{:?}", std::env::current_dir().unwrap());
    let rom = libgameboii::open_rom(&"C:\\lol.gb").unwrap();
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
