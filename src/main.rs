extern crate bit_field;
extern crate clap;
extern crate glutin_window;
extern crate graphics;
extern crate image;
extern crate opengl_graphics;
extern crate piston;
extern crate regex;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod address;
mod cpu;
mod debug_log;
mod function_stubs;
mod interpreter;
mod ppu;
mod window;

use clap::{App, Arg};
use cpu::CPU;
use debug_log::Log;
use opengl_graphics::OpenGL;
use piston::input::*;
use ppu::PPU;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

fn open_rom<P: AsRef<Path>>(rom_path: &P) -> std::io::Result<Vec<u8>> {
    let mut content = vec![];
    File::open(rom_path)?.read_to_end(&mut content)?;

    //TODO unzip?

    Ok(content)
}

fn dump_ram(ram: &[u8]) -> std::io::Result<()> {
    let mut file = File::create("ramdump.bin")?;

    file.write(ram)?;

    Ok(())
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
        .arg(
            Arg::with_name("speed_mult")
                .long("speed_mult")
                .short("m")
                .takes_value(true)
                .default_value("1")
                .help("A clock multiplier to speed up emulation"),
        )
        .arg(
            Arg::with_name("headless")
                .long("headless")
                .help("The emulator won't create a window if true. Useful for tests"),
        )
        .get_matches();

    //load the file from command line
    let rom_path = matches.value_of("ROMFILE").unwrap();

    let speed_mult = match matches.value_of("speed_mult").unwrap().parse::<u64>() {
        Ok(num) => num,
        Err(e) => {
            println!("Invalid value for speed_mult");
            println!("{}", e);
            std::process::exit(1);
        }
    };

    let rom = open_rom(&rom_path).unwrap_or_else(|error| {
        println!("Cannot open file: {}", rom_path);
        println!("An error occurred:");
        println!("{}", error);
        std::process::exit(1);
    });

    let do_log = matches.is_present("debug_log");
    let headless = matches.is_present("headless");

    let mut log = if do_log { Some(Log::new()) } else { None };
    let mut ppu = PPU::new();
    let mut cpu = CPU::new(&rom);

    let mut paused = false;
    let mut current_clock = 0;

    let mut update = |cpu: &mut CPU, ppu: &mut PPU| {
        cpu.tick(current_clock, &mut log);
        ppu.tick(cpu, current_clock);

        current_clock += 1;

        !cpu.should_exit
    };

    if headless {
        println!("Running headless");
        while update(&mut cpu, &mut ppu) {}
    } else {
        // Create an Glutin window.
        let mut window = window::Window::new(OpenGL::V3_2);

        while let Some(e) = window.next() {
            if let Some(ue) = e.update_args() {
                let clocks = (cpu::MACHINE_HZ as f64 * ue.dt) as u64 * speed_mult;
                for _ in 0..clocks {
                    if !update(&mut cpu, &mut ppu) {
                        return;
                    }
                }
            }

            if let Some(r) = e.render_args() {
                window.render(&r, &ppu);
            }

            if let Some(i) = e.button_args() {
                if i.state == ButtonState::Press {
                    match i.button {
                        Button::Keyboard(k) => {
                            if k == keyboard::Key::F5 {
                                paused = !paused;
                            } else if k == keyboard::Key::F1 {
                                dump_ram(&cpu.RAM).unwrap();
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}
