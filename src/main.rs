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

mod cpu;
mod debug_log;
mod function_stubs;
mod gpu;
mod interpreter;

use clap::{App, Arg};
use cpu::CPU;
use glutin_window::GlutinWindow;
use gpu::GPU;
use opengl_graphics::OpenGL;
use piston::event_loop::*;
use piston::input::*;
use piston::window::Window;
use piston::window::WindowSettings;
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

    // Create an Glutin window.
    let scale = 4;
    let gl_version = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new(
        "worldgentest",
        [gpu::RESOLUTION_W * scale, gpu::RESOLUTION_H * scale],
    ).opengl(gl_version)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gpu = GPU::new(gl_version);
    let mut cpu = CPU::new(&rom, do_log);

    let mut events = Events::new(EventSettings {
        max_fps: 60,
        ups: cpu::MACHINE_HZ, //TODO it would be neat to use events to push the clock and the gpu
        bench_mode: false,
        lazy: false,
        swap_buffers: true,
        ups_reset: 0,
    });

    while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.update_args() {
            cpu.tick();
            gpu.tick(&mut cpu);
        }

        if let Some(r) = e.render_args() {
            gpu.render(&r);
        }
    }
}
