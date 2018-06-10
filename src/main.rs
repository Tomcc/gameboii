extern crate bit_field;
extern crate clap;
extern crate orbclient;
extern crate regex;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod cpu;
mod debug_log;
mod function_stubs;
mod gpu;
mod interpreter;

use clap::{App, Arg};
use cpu::CPU;
use gpu::GPU;
use std::fs::File;
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
                .required(true),
        )
        .arg(
            Arg::with_name("debug_log")
                .long("log")
                .help("Write the executable map and the log to file. Very slow"),
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

    let do_log = matches.is_present("debug_log");

    let mut gpu = GPU::new();
    let mut cpu = CPU::new(&rom, do_log);

    loop {
        cpu.tick();
        gpu.tick(&mut cpu);
    }
}
