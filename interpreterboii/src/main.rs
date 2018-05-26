extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::io::Write;

#[derive(Deserialize, Debug)]
struct OpCodeDesc {
    prefix: Option<String>,
    opcode: String,
    mnemonic: String,
    operands: Vec<String>,
    bytes: usize,
    cycles: usize,
    flagsZNHC: Vec<String>,
}

fn write_flag_handler(outfile: &mut File, name: &str, value: &str) -> std::io::Result<()> {
    if value == "0" {
        writeln!(outfile, "\t\t\tcpu.set_{}();", name)?;
    } else if value == "1" {
        writeln!(outfile, "\t\t\tcpu.reset_{}();", name)?;
    }
    Ok(())
}

fn write_flag_overload(outfile: &mut File, name: &str, value: &str) -> std::io::Result<()> {
    if value != "0" && value != "1" && value != "-" {
        write!(outfile, "_{}", name)?;
    }
    Ok(())
}

enum HLMagic {
    Inc,
    Dec,
}

fn parse_operand(operand: &str) -> (String, Option<HLMagic>) {
    if operand.starts_with("(") {
        let len = operand.chars().count();
        let inner: String = operand.chars().skip(1).take(len - 2).collect();
        let (op, hl) = parse_operand(&inner);
        return (format!("cpu.address({})", op), hl);
    } else if operand == "HL+" {
        return (String::from("cpu.HL.HL"), Some(HLMagic::Inc));
    } else if operand == "HL-" {
        return (String::from("cpu.HL.HL"), Some(HLMagic::Dec));
    } else if operand.contains('+') {
        let operands: Vec<&str> = operand.split('+').collect();
        let (mut op1, hl1) = parse_operand(&operands[0]);
        let (op2, _) = parse_operand(&operands[1]);

        op1 += " + ";
        op1 += &op2;
        return (op1, hl1);
    } else if operand == "d8" {
        return (format!("cpu.immediateU8()"), None);
    } else if operand == "d16" {
        return (format!("cpu.immediateU16()"), None);
    } else if operand == "a8" {
        return (format!("cpu.immediateU8()"), None);
    } else if operand == "a16" {
        return (format!("cpu.immediateU16()"), None);
    } else if operand == "r8" {
        return (format!("cpu.immediateI8()"), None);
    } else if let Ok(num) = operand.parse::<usize>() {
        return (format!("{}", num), None);
    } else if operand.ends_with("H") && operand.len() == 3 {
        let num: String = operand.chars().take(2).collect();
        return (format!("0x{}", num), None);
    } else if operand == "A" {
        return (String::from("cpu.AF.A"), None);
    } else if operand == "F" {
        return (String::from("cpu.AF.F"), None);
    } else if operand == "B" {
        return (String::from("cpu.BC.B"), None);
    } else if operand == "C" {
        return (String::from("cpu.BC.C"), None);
    } else if operand == "D" {
        return (String::from("cpu.DE.D"), None);
    } else if operand == "E" {
        return (String::from("cpu.DE.E"), None);
    } else if operand == "H" {
        return (String::from("cpu.HL.H"), None);
    } else if operand == "L" {
        return (String::from("cpu.HL.L"), None);
    } else if operand == "SP" {
        return (String::from("cpu.SP"), None);
    } else if operand == "PC" {
        return (String::from("cpu.PC"), None);
    } else {
        return (format!("cpu.{0}.{0}", operand), None);
    }
}

fn write_operand(outfile: &mut File, operand: &str) -> std::io::Result<Option<HLMagic>> {
    let (op, hl) = parse_operand(operand);
    write!(outfile, "{}", op)?;
    Ok(hl)
}

fn write_rust_opcodes(opcodes: &[OpCodeDesc]) -> std::io::Result<()> {
    let outfile = &mut File::create("../src/interpreter.rs")?;

    writeln!(
        outfile,
        "fn interpret(cpu: &mut CPU, instr: usize) unsafe {{"
    )?;
    writeln!(outfile, "\tmatch instr {{")?;

    for opcode in opcodes {
        if let Some(_) = opcode.prefix {
            let truncated: String = opcode.opcode.chars().skip(2).collect();
            writeln!(outfile, "\t\t0xcb{} => {{", truncated)?;
        } else {
            writeln!(outfile, "\t\t{} => {{", opcode.opcode)?;
        }
        write!(outfile, "\t\t\tcpu.{}", opcode.mnemonic)?;

        write_flag_overload(outfile, "z", &opcode.flagsZNHC[0])?;
        write_flag_overload(outfile, "n", &opcode.flagsZNHC[1])?;
        write_flag_overload(outfile, "h", &opcode.flagsZNHC[2])?;
        write_flag_overload(outfile, "c", &opcode.flagsZNHC[3])?;

        write!(outfile, "(")?;

        let mut first = true;
        let mut hl_magic: Option<HLMagic> = None;
        for operand in &opcode.operands {
            if !first {
                write!(outfile, ", ")?;
            } else {
                first = false;
            }
            if let Some(hl) = write_operand(outfile, operand)? {
                hl_magic = Some(hl);
            }
        }

        //write the parameters
        writeln!(outfile, ");")?;

        //do the HL magic
        if let Some(hl) = hl_magic {
            match hl {
                HLMagic::Inc => {
                    writeln!(outfile, "\t\t\t{} += 1;", parse_operand("HL").0)?;
                }
                HLMagic::Dec => {
                    writeln!(outfile, "\t\t\t{} -= 1;", parse_operand("HL").0)?;
                }
            }
        }

        //set known flags
        write_flag_handler(outfile, "z", &opcode.flagsZNHC[0])?;
        write_flag_handler(outfile, "n", &opcode.flagsZNHC[1])?;
        write_flag_handler(outfile, "h", &opcode.flagsZNHC[2])?;
        write_flag_handler(outfile, "c", &opcode.flagsZNHC[3])?;

        //advance the program counter
        writeln!(outfile, "\t\t\tcpu.PC += {};", opcode.bytes)?;

        //cycle
        writeln!(outfile, "\t\t\tcpu.run_cycles({});", opcode.cycles)?;

        writeln!(outfile, "\t\t}},")?;
    }

    writeln!(outfile, "    }}")?;
    writeln!(outfile, "}}")?;

    Ok(())
}

fn main() {
    let opcodes: Vec<OpCodeDesc> =
        serde_json::from_reader(File::open("opcodes.json").unwrap()).unwrap();
    write_rust_opcodes(&opcodes);
}
