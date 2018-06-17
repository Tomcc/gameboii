extern crate std;

extern crate serde;
extern crate serde_json;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::Write;
use std::time::Duration;
use std::time::Instant;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct OpCodeDesc {
    mnemonic: String,
    operands: Vec<String>,
    bytes: Option<usize>,
    cycles: Option<usize>,
    flagsZNHC: Vec<String>,
}

impl OpCodeDesc {
    fn shorthand(&self, immediate: &[u8]) -> String {
        let mut line = String::new();
        let mnemonic = &self.mnemonic;
        line += mnemonic;

        for op in &self.operands {
            let mut op = op.clone();

            if op.contains("inout ") {
                op = op.replace("inout ", "");
            } else if op.contains("out ") {
                op = op.replace("out ", "");
            }
            if op.contains("d8") {
                op = op.replace("d8", &format!("0x{:02x}", immediate[0]));
            }
            if op.contains("d16") {
                op = op.replace(
                    "d16",
                    &format!(
                        "0x{:04x}",
                        ((immediate[1] as u16) << 8) | (immediate[0] as u16)
                    ),
                );
            } else if op.contains("a16") {
                op = op.replace(
                    "a16",
                    &format!(
                        "0x{:04x}",
                        ((immediate[1] as u16) << 8) | (immediate[0] as u16)
                    ),
                );
            }
            if op.contains("r8") {
                op = op.replace(
                    "r8",
                    &format!("{:x}", unsafe {
                        std::mem::transmute::<u8, i8>(immediate[0])
                    }),
                );
            }
            if op.contains("a8") {
                op = op.replace("a8", &format!("0xff00 + 0x{:02x}", immediate[0]));
            }

            line += " ";
            line += &op;
        }

        line
    }
}

pub struct Log {
    disasm_file: File,
    opcodes: BTreeMap<String, OpCodeDesc>,
    disassembly: BTreeMap<usize, String>,
    next_write_time: Instant,
}

impl Drop for Log {
    fn drop(&mut self) {
        self.write_to_file().unwrap(); //can't return an error here
    }
}

impl Log {
    pub fn new() -> Self {
        Log {
            disasm_file: File::create("disasm_file.txt").unwrap(),
            opcodes: serde_json::from_reader(File::open("opcodes.json").unwrap()).unwrap(),
            disassembly: BTreeMap::new(),
            next_write_time: Instant::now(),
        }
    }

    fn write_to_file(&mut self) -> std::io::Result<()> {
        self.disasm_file.seek(SeekFrom::Start(0))?;
        let mut last_addr = 0;
        for (addr, text) in &self.disassembly {
            if *addr > last_addr + 4 {
                writeln!(self.disasm_file, "")?;
                writeln!(self.disasm_file, "...")?;
                writeln!(self.disasm_file, "")?;
            }

            writeln!(self.disasm_file, "{}", text)?;
            last_addr = *addr;
        }

        self.next_write_time += Duration::from_secs(1);
        Ok(())
    }

    pub fn log_instruction(
        &mut self,
        instr: u16,
        immediate: &[u8],
        pc: usize,
    ) -> std::io::Result<()> {
        //compose the line
        {
            let mut line = format!("{:04x}\t", pc);
            let text_instruction = format!("0x{:02x}", instr);

            println!("{}", text_instruction);

            let opcode = &self.opcodes[&text_instruction];

            line += &opcode.shorthand(immediate);

            self.disassembly.insert(pc, line);
        }
        if Instant::now() > self.next_write_time {
            self.write_to_file()?;
        }

        Ok(())
    }
}
