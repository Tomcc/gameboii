extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

const HEADER: &str = r#"
use cpu::CPU;

pub unsafe fn interpret(cpu: &mut CPU, instr: usize) {{
"#;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
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

fn maybe_flag(name: &str, value: &str) -> String {
    if value != "0" && value != "1" && value != "-" {
        return String::from("_") + name;
    }
    String::new()
}

enum HLMagic {
    Inc,
    Dec,
}

fn make_operand(operand: &str) -> (String, Option<HLMagic>) {
    if operand.starts_with("(") {
        let len = operand.chars().count();
        let inner: String = operand.chars().skip(1).take(len - 2).collect();
        let (op, hl) = make_operand(&inner);
        return (format!("cpu.address({})", op), hl);
    } else if operand == "HL+" {
        return (String::from("cpu.HL.HL"), Some(HLMagic::Inc));
    } else if operand == "HL-" {
        return (String::from("cpu.HL.HL"), Some(HLMagic::Dec));
    } else if operand.contains('+') {
        let operands: Vec<&str> = operand.split('+').collect();
        let (mut op1, hl1) = make_operand(&operands[0]);
        let (op2, _) = make_operand(&operands[1]);

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
        return (String::from("cpu.AF.r8.0"), None);
    } else if operand == "F" {
        return (String::from("cpu.AF.r8.1"), None);
    } else if operand == "B" {
        return (String::from("cpu.BC.r8.0"), None);
    } else if operand == "C" {
        return (String::from("cpu.BC.r8.1"), None);
    } else if operand == "D" {
        return (String::from("cpu.DE.r8.0"), None);
    } else if operand == "E" {
        return (String::from("cpu.DE.r8.1"), None);
    } else if operand == "H" {
        return (String::from("cpu.HL.r8.0"), None);
    } else if operand == "L" {
        return (String::from("cpu.HL.r8.1"), None);
    } else if operand == "SP" {
        return (String::from("cpu.SP"), None);
    } else if operand == "PC" {
        return (String::from("cpu.PC"), None);
    } else {
        return (format!("cpu.{0}.r16", operand), None);
    }
}

enum ParameterType {
    u16,
    u8,
}

struct FunctionParameter {
    mutable: bool,
    param_type: ParameterType,
}

struct FunctionDesc {
    name: String,
    hl: Option<HLMagic>,
    parameters: Vec<FunctionParameter>,
    fullcode: String,
}

impl FunctionDesc {
    pub fn from_opcode(opcode: &OpCodeDesc) -> Self {
        let mut res = String::from("\t\t\tcpu.");
        let mut name = String::new();

        name += &opcode.mnemonic;
        name += &maybe_flag("z", &opcode.flagsZNHC[0]);
        name += &maybe_flag("n", &opcode.flagsZNHC[1]);
        name += &maybe_flag("h", &opcode.flagsZNHC[2]);
        name += &maybe_flag("c", &opcode.flagsZNHC[3]);

        res += &name;
        res += "(";

        let mut first = true;
        let mut hl_magic: Option<HLMagic> = None;
        for operand in &opcode.operands {
            if !first {
                res += ", ";
            } else {
                first = false;
            }
            let (op, maybe_hl) = make_operand(operand);
            res += &op;
            if let Some(hl) = maybe_hl {
                hl_magic = Some(hl);
            }
        }

        //write the parameters
        res += ");";

        FunctionDesc {
            name: name,
            hl: hl_magic,
            fullcode: res,
            parameters: vec![],
        }
    }
}

fn write_rust_opcodes(opcodes: &[OpCodeDesc]) -> std::io::Result<Vec<FunctionDesc>> {
    let outfile = &mut File::create("../src/interpreter.rs")?;
    let mut function_list = vec![];

    writeln!(outfile, "{}", HEADER)?;
    writeln!(outfile, "\tmatch instr {{")?;

    for opcode in opcodes {
        if let Some(_) = opcode.prefix {
            let truncated: String = opcode.opcode.chars().skip(2).collect();
            writeln!(outfile, "\t\t0xcb{} => {{", truncated)?;
        } else {
            writeln!(outfile, "\t\t{} => {{", opcode.opcode)?;
        }

        let function = FunctionDesc::from_opcode(opcode);

        writeln!(outfile, "{}", &function.fullcode)?;

        //do the HL magic
        if let Some(hl) = &function.hl {
            match hl {
                HLMagic::Inc => {
                    writeln!(outfile, "\t\t\t{} += 1;", make_operand("HL").0)?;
                }
                HLMagic::Dec => {
                    writeln!(outfile, "\t\t\t{} -= 1;", make_operand("HL").0)?;
                }
            }
        }

        //add the function to the list
        function_list.push(function);

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

    Ok(function_list)
}

fn write_function_stub(outfile: &mut File, function: &FunctionDesc) -> std::io::Result<()> {
    writeln!(
        outfile,
        r#"
    pub fn {}(&mut self) {{
        panic!("not implemented");
    }}"#,
        function.name
    )?;

    Ok(())
}

fn write_function_stubs(functions: &[FunctionDesc]) -> std::io::Result<()> {
    //write out stubs for the functions that were found
    let mut dedupd = BTreeMap::new();

    for func in functions {
        dedupd.insert(func.name.clone(), func);
    }

    //open the output file
    let outfile = &mut File::create("../src/function_stubs.rs")?;

    write!(
        outfile,
        r#"
use cpu::CPU;

impl CPU {{
"#
    )?;

    //write out all the remaining functions in alphabetical order
    for func in dedupd.values() {
        write_function_stub(outfile, func);
    }
    writeln!(outfile, "}}")?;

    Ok(())
}

fn main() {
    let opcodes: Vec<OpCodeDesc> =
        serde_json::from_reader(File::open("opcodes.json").unwrap()).unwrap();
    match write_rust_opcodes(&opcodes) {
        Ok(functions) => {
            write_function_stubs(&functions).unwrap();
            std::process::exit(0);
        }
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
