extern crate std;

extern crate serde_json;
extern crate serde;

use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::Write;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct OpCodeDesc {
    prefix: Option<String>,
    opcode: String,
    mnemonic: String,
    operands: Vec<String>,
    bytes: Option<usize>,
    cycles: Option<usize>,
    flagsZNHC: Vec<String>,
}

pub struct Log {
    // exec_log: File,
    map_log: File,
    opcodes: Vec<OpCodeDesc>,
}

impl Log {
    pub fn new() -> Self {
        Log {
            // exec_log: File::create("exec_log.txt").unwrap(),
            map_log: File::create("map_log.txt").unwrap(),
            opcodes: serde_json::from_reader(File::open("interpreterboii/opcodes.json").unwrap()).unwrap(),
        }
    }

    pub fn log_instruction(&mut self, instr: u8, pc: u16) -> std::io::Result<()> {
        if instr == 0xcb {
            return Ok(()); //avoid the prefix
        }

        //compose the line
        let mut line = format!("{:04x}\t", pc);

        let mnemonic = &self.opcodes[instr as usize].mnemonic;
        line += mnemonic;
        //append spaces to make it 4 chars
        for _ in mnemonic.len()..4 {
            line += " ";
        }
        line += "\n";

        let line_index = pc as u64 * line.len() as u64;
        let line_end = pc as u64 * (line.len() + 1) as u64;

        //expand the file as needed
        while self.map_log.metadata()?.len() < line_end {
            let mut v = vec![' ' as u8; line.len()];
            v[line.len() - 1] = '\n' as u8;
            self.map_log.write(&v)?;
        }

        //seek to this entry line
        self.map_log.seek(SeekFrom::Start(line_index))?;

        write!(self.map_log, "{}", line)?;

        // self.map_log.sync_all()?;

        Ok(())
    }
}
