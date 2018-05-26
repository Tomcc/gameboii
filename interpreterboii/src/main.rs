extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::fmt;

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

#[derive(Clone, Copy)]
enum HLMagic {
    Inc,
    Dec,
}

enum ParameterType {
    U16,
    U8,
}

impl fmt::Display for ParameterType {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParameterType::U8 => write!(f, "u8"),
            ParameterType::U16 => write!(f, "u16"),
        }
    }
}

struct Parameter {
    mutable: bool,
    param_type: ParameterType,
    hl: Option<HLMagic>,
    fullcode: String,
}

impl fmt::Display for Parameter {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.param_type)
    }
}

impl Parameter {
    fn new(code: String, t: ParameterType) -> Self {
        Parameter {
            fullcode: code,
            param_type: t,
            hl: None,
            mutable: false,
        }
    }

    fn from_operand(operand: &str) -> Self {
        if operand.starts_with("(") {
            let len = operand.chars().count();
            let inner: String = operand.chars().skip(1).take(len - 2).collect();
            let param = Self::from_operand(&inner);
            return Parameter {
                fullcode: format!("cpu.address({})", param.fullcode),
                hl: param.hl,
                param_type: ParameterType::U8,
                mutable: false,
            };
        } else if operand == "HL+" {
            return Parameter {
                fullcode: String::from("cpu.HL.HL"),
                hl: Some(HLMagic::Inc),
                param_type: ParameterType::U16,
                mutable: false,
            };
        } else if operand == "HL-" {
            return Parameter {
                fullcode: String::from("cpu.HL.HL"),
                hl: Some(HLMagic::Dec),
                param_type: ParameterType::U16,
                mutable: false,
            };
        } else if operand.contains('+') {
            let operands: Vec<&str> = operand.split('+').collect();
            let param1 = Self::from_operand(&operands[0]);
            let param2 = Self::from_operand(&operands[1]);

            return Parameter {
                fullcode: param1.fullcode + " + " + &param2.fullcode,
                param_type: ParameterType::U8,
                hl: None,
                mutable: false,
            };
        } else if operand == "d8" {
            return Parameter::new(format!("cpu.immediateU8()"), ParameterType::U8);
        } else if operand == "d16" {
            return Parameter::new(format!("cpu.immediateU16()"), ParameterType::U16);
        } else if operand == "a8" {
            return Parameter::new(format!("cpu.immediateU8()"), ParameterType::U8);
        } else if operand == "a16" {
            return Parameter::new(format!("cpu.immediateU16()"), ParameterType::U16);
        } else if operand == "r8" {
            return Parameter::new(format!("cpu.immediateI8()"), ParameterType::U8);
        } else if let Ok(num) = operand.parse::<usize>() {
            return Parameter::new(format!("{}", num), ParameterType::U8);
        } else if operand.ends_with("H") && operand.len() == 3 {
            let num: String = operand.chars().take(2).collect();
            return Parameter::new(format!("0x{}", num), ParameterType::U16);
        } else if operand == "A" {
            return Parameter::new(String::from("cpu.AF.r8.0"), ParameterType::U8);
        } else if operand == "F" {
            return Parameter::new(String::from("cpu.AF.r8.1"), ParameterType::U8);
        } else if operand == "B" {
            return Parameter::new(String::from("cpu.BC.r8.0"), ParameterType::U8);
        } else if operand == "C" {
            return Parameter::new(String::from("cpu.BC.r8.1"), ParameterType::U8);
        } else if operand == "D" {
            return Parameter::new(String::from("cpu.DE.r8.0"), ParameterType::U8);
        } else if operand == "E" {
            return Parameter::new(String::from("cpu.DE.r8.1"), ParameterType::U8);
        } else if operand == "H" {
            return Parameter::new(String::from("cpu.HL.r8.0"), ParameterType::U8);
        } else if operand == "L" {
            return Parameter::new(String::from("cpu.HL.r8.1"), ParameterType::U8);
        } else if operand == "SP" {
            return Parameter::new(String::from("cpu.SP"), ParameterType::U16);
        } else if operand == "PC" {
            return Parameter::new(String::from("cpu.PC"), ParameterType::U16);
        } else {
            return Parameter::new(format!("cpu.{0}.r16", operand), ParameterType::U16);
        }
    }
}

struct FunctionDesc {
    name: String,
    parameters: Vec<Parameter>,
    fullcode: String,
}

impl FunctionDesc {
    pub fn from_opcode(opcode: &OpCodeDesc) -> Self {
        let mut code = String::from("\t\t\tcpu.");
        let mut name = String::new();
        let mut parameters = vec![];

        name += &opcode.mnemonic;
        name += &maybe_flag("z", &opcode.flagsZNHC[0]);
        name += &maybe_flag("n", &opcode.flagsZNHC[1]);
        name += &maybe_flag("h", &opcode.flagsZNHC[2]);
        name += &maybe_flag("c", &opcode.flagsZNHC[3]);

        code += &name;
        code += "(";

        let mut first = true;
        for operand in &opcode.operands {
            if !first {
                code += ", ";
            } else {
                first = false;
            }
            let param = Parameter::from_operand(operand);
            code += &param.fullcode;
            parameters.push(param);
        }

        //write the parameters
        code += ");";

        FunctionDesc {
            name: name,
            fullcode: code,
            parameters: parameters,
        }
    }

    fn hl_mode(&self) -> Option<HLMagic> {
        for param in &self.parameters {
            if param.hl.is_some() {
                return param.hl;
            }
        }
        None
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
        if let Some(hl) = &function.hl_mode() {
            match hl {
                HLMagic::Inc => {
                    writeln!(
                        outfile,
                        "\t\t\t{} += 1;",
                        Parameter::from_operand("HL").fullcode
                    )?;
                }
                HLMagic::Dec => {
                    writeln!(
                        outfile,
                        "\t\t\t{} -= 1;",
                        Parameter::from_operand("HL").fullcode
                    )?;
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

    //compose the parameters
    let mut paramcode = String::new();
    for idx in 0..function.parameters.len() {
        let param = &function.parameters[idx];

        paramcode += &format!(", reg{}: {}", idx, param);
    }

    writeln!(
        outfile,
        r#"
    pub fn {}(&mut self{}) {{
        panic!("not implemented");
    }}"#,
        function.name,
        paramcode,
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
