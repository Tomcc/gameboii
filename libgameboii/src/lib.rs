extern crate bit_field;
extern crate image;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod address;
pub mod cpu;
pub mod debug_log;
mod function_stubs;
pub mod interpreter;
pub mod ppu;

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn open_rom<P: AsRef<Path>>(rom_path: &P) -> std::io::Result<Vec<u8>> {
    let mut content = vec![];
    File::open(rom_path)?.read_to_end(&mut content)?;

    //TODO unzip?

    Ok(content)
}
