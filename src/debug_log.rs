extern crate std;

extern crate serde;
extern crate serde_json;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::Write;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct OpCodeDesc {
    mnemonic: String,
    operands: Vec<String>,
    bytes: Option<usize>,
    cycles: Option<usize>,
    flagsZNHC: Vec<String>,
}

pub struct Log {
    disasm_file: File,
    opcodes: BTreeMap<String, OpCodeDesc>,
}

impl Log {
    pub fn new() -> Self {
        Log {
            disasm_file: File::create("disasm_file.txt").unwrap(),
            opcodes: serde_json::from_reader(File::open("opcodes.json").unwrap()).unwrap(),
        }
    }

    pub fn log_instruction(&mut self, instr: u8, pc: u16) -> std::io::Result<()> {
        //compose the line
        let mut line = format!("{:04x}\t", pc);
        let text_instruction = format!("0x{:02x}", instr);

        let mnemonic = &self.opcodes[&text_instruction].mnemonic;
        line += mnemonic;
        //append spaces to make it 4 chars
        for _ in mnemonic.len()..4 {
            line += " ";
        }
        line += "\n";

        let line_index = pc as u64 * line.len() as u64;
        let line_end = pc as u64 * (line.len() + 1) as u64;

        //expand the file as needed
        while self.disasm_file.metadata()?.len() < line_end {
            let mut v = vec![' ' as u8; line.len()];
            v[line.len() - 1] = '\n' as u8;
            self.disasm_file.write(&v)?;
        }

        //seek to this entry line
        self.disasm_file.seek(SeekFrom::Start(line_index))?;

        write!(self.disasm_file, "{}", line)?;

        Ok(())
    }
}
