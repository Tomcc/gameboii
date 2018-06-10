extern crate bit_field;
extern crate clap;

mod cpu;
mod function_stubs;
mod interpreter;

use std::fs::File;
use clap::{App, Arg};
use cpu::CPU;
use std::io::Read;
use std::path::Path;

fn open_rom<P: AsRef<Path>>(rom_path: &P) -> std::io::Result<Vec<u8>> {
    let mut content = vec![];
    File::open(rom_path)?.read_to_end(&mut content)?;

    //TODO unzip?

    Ok(content)
}

fn main() {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    let matches = App::new("GAMEBOII")
        .version(VERSION)
        .about("It plays the gameboy dance")
        .arg(
            Arg::with_name("ROMFILE")
                .value_name("FILE")
                .help("Set the cartridge ROM to load")
                .takes_value(true)
                .required(true)
        )
        .get_matches();

    //load the file from command line
    let rom_path = matches.value_of("ROMFILE").unwrap();
    
    let rom = open_rom(&rom_path).unwrap_or_else(|error| {
        println!("Cannot open file: {}", rom_path);
        println!("An error occurred:");
        println!("{}", error);
        std::process::exit(1);
    });
    
    let mut cpu = CPU::new(&rom);

    cpu.run();
}
