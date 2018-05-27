extern crate itertools;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::Write;

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
    Bool,
    I8,
}

impl fmt::Display for ParameterType {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParameterType::U8 => write!(f, "u8"),
            ParameterType::I8 => write!(f, "i8"),
            ParameterType::U16 => write!(f, "u16"),
            ParameterType::Bool => write!(f, "bool"),
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

            // when the inner parameter is 8-bit, we need to add it to 0xff00.
            // for reasons.
            let post_offset_code = match param.param_type {
                ParameterType::I8 | ParameterType::U8 => param.fullcode + " as u16 + 0xff00",
                _ => param.fullcode,
            };

            return Parameter {
                fullcode: format!("cpu.address({})", post_offset_code),
                hl: param.hl,
                param_type: ParameterType::U8,
                mutable: false,
            };
        } else if operand == "HL+" {
            let mut param = Self::from_operand("HL");
            param.hl = Some(HLMagic::Inc);
            return param;
        } else if operand == "HL-" {
            let mut param = Self::from_operand("HL");
            param.hl = Some(HLMagic::Dec);
            return param;
        } else if operand == "SP+r8" {
            let param = Self::from_operand("r8");
            return Parameter::new(
                format!("cpu.offset_sp({})", param.fullcode),
                ParameterType::U16,
            );
        } else if operand == "d8" {
            return Parameter::new(format!("cpu.immediateU8()"), ParameterType::U8);
        } else if operand == "d16" {
            return Parameter::new(format!("cpu.immediateU16()"), ParameterType::U16);
        } else if operand == "a8" {
            return Parameter::new(format!("cpu.immediateU8()"), ParameterType::U8);
        } else if operand == "a16" {
            return Parameter::new(format!("cpu.immediateU16()"), ParameterType::U16);
        } else if operand == "r8" {
            return Parameter::new(format!("cpu.immediateI8()"), ParameterType::I8);
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
        } else if operand == "L" {
            return Parameter::new(String::from("cpu.HL.r8.1"), ParameterType::U8);
        } else if operand == "Z" {
            return Parameter::new(String::from("cpu.z()"), ParameterType::Bool);
        } else if operand == "NZ" {
            return Parameter::new(String::from("!cpu.z()"), ParameterType::Bool);
        } else if operand == "C" {
            return Parameter::new(String::from("cpu.c()"), ParameterType::Bool);
        } else if operand == "NC" {
            return Parameter::new(String::from("!cpu.c()"), ParameterType::Bool);
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
        let mut code = String::new();
        let mut name = String::new();
        let mut parameters = vec![];

        for operand in &opcode.operands {
            parameters.push(Parameter::from_operand(operand));
        }

        //get the parameters in their own line to help borrowck not confuse itself and die
        for idx in 0..parameters.len() {
            code += &format!("\t\t\tlet reg{} = {};\n", idx, parameters[idx].fullcode);
        }

        name += &format!("\t\t\tcpu.{}", opcode.mnemonic);
        name += &maybe_flag("z", &opcode.flagsZNHC[0]);
        name += &maybe_flag("n", &opcode.flagsZNHC[1]);
        name += &maybe_flag("h", &opcode.flagsZNHC[2]);
        name += &maybe_flag("c", &opcode.flagsZNHC[3]);

        //append the types to do overloading
        for parameter in &parameters {
            name += &format!("_{}", parameter.param_type);
        }

        code += &name;
        code += "(";

        for idx in 0..parameters.len() {
            if idx > 0 {
                code += ", ";
            }
            code += &format!("reg{}", idx);
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

const HEADER: &str = r#"
use cpu::CPU;

pub unsafe fn interpret(cpu: &mut CPU) {
    match cpu.peek_instruction() {
"#;

const FUNC_SPLIT: &str = r#"
        _ => panic!("instruction not known")
    }
}

pub unsafe fn interpret_cb(cpu: &mut CPU, instr: u8) {
    match cpu.peek_instruction() {
"#;

const FOOTER: &str = r#"
        _ => panic!("instruction not known")
    }
}"#;

fn write_opcodes(outfile: &mut File, opcodes: &[OpCodeDesc]) -> std::io::Result<Vec<FunctionDesc>> {
    let mut function_list = vec![];

    for opcode in opcodes {
        writeln!(outfile, "\t\t{} => {{", opcode.opcode)?;

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
    Ok(function_list)
}

fn write_interpreter(mut opcodes: Vec<OpCodeDesc>) -> std::io::Result<Vec<FunctionDesc>> {
    let outfile = &mut File::create("../src/interpreter.rs")?;
    
    //sort the opcodes between cb and non cb
    let splitpoint = itertools::partition(&mut opcodes, |opcode| opcode.prefix.is_none());

    writeln!(outfile, "{}", HEADER)?;

    let mut function_list = write_opcodes(outfile, &opcodes[..splitpoint])?;

    writeln!(outfile, "{}", FUNC_SPLIT)?;

    function_list.append(&mut write_opcodes(outfile, &opcodes[splitpoint..])?);

    writeln!(outfile, "{}", FOOTER)?;

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
        function.name, paramcode,
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
    match write_interpreter(opcodes) {
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
