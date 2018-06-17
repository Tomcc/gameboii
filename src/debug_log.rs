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

const FIXED_LINE_LEN: usize = 100;

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

    pub fn log_instruction(
        &mut self,
        instr: u16,
        immediate: &[u8],
        pc: usize,
    ) -> std::io::Result<()> {
        //compose the line
        let mut line = format!("{:04x}\t", pc);
        let text_instruction = format!("0x{:02x}", instr);

        println!("{}", text_instruction);

        let opcode = &self.opcodes[&text_instruction];

        line += &opcode.shorthand(immediate);

        assert!(line.len() <= FIXED_LINE_LEN);

        //append spaces to make it 10 chars
        for _ in line.len()..FIXED_LINE_LEN {
            line += " ";
        }
        line += "\n";

        let line_index = pc * line.len();
        let line_end = pc * (line.len() + 1);

        //expand the file as needed
        while self.disasm_file.metadata()?.len() < line_end as u64 {
            let mut v = vec![' ' as u8; line.len()];
            v[line.len() - 1] = '\n' as u8;
            self.disasm_file.write(&v)?;
        }

        //seek to this entry line
        self.disasm_file.seek(SeekFrom::Start(line_index as u64))?;

        write!(self.disasm_file, "{}", line)?;

        Ok(())
    }
}
