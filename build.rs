extern crate itertools;
extern crate regex;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use regex::Regex;
use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct OpCodeDesc {
    mnemonic: String,
    operands: Vec<String>,
    bytes: usize,
    cycles: usize,
    flagsZNHC: Vec<String>,
}

struct FunctionCode {
    lines: Vec<String>,
}

impl FunctionCode {
    fn new() -> Self {
        FunctionCode { lines: vec![] }
    }

    fn not_found(opcode: &str) -> Self {
        FunctionCode {
            lines: vec![format!("\t\tpanic!(\"{} not implemented\");", opcode)],
        }
    }
}

type FunctionCodeMap = BTreeMap<String, FunctionCode>;

fn write_flag_handler(outfile: &mut File, name: &str, value: &str) -> std::io::Result<()> {
    if value == "0" {
        writeln!(outfile, "\t\t\tcpu.set_{}(false);", name)?;
    } else if value == "1" {
        writeln!(outfile, "\t\t\tcpu.set_{}(true);", name)?;
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

#[derive(Clone)]
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

#[derive(Clone)]
enum OutputMode {
    None,
    Out,
    Inout,
}

#[derive(Clone)]
struct Parameter {
    output: OutputMode,
    param_type: ParameterType,
    hl: Option<HLMagic>,
    fullcode: String,
    inner: Option<Box<Parameter>>,
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
            output: OutputMode::None,
            inner: None,
        }
    }

    fn immediate(t: ParameterType) -> Self {
        Parameter {
            fullcode: String::from("imm0"),
            param_type: t,
            hl: None,
            output: OutputMode::None,
            inner: None,
        }
    }

    fn write_immediate_load(&self, outfile: &mut File) -> std::io::Result<()> {
        if let Some(ref inner) = self.inner {
            return inner.write_immediate_load(outfile);
        }

        //a bit hacky to check if immediate like this, but yeah...
        if self.fullcode == "imm0" {
            return writeln!(
                outfile,
                "\t\t\tlet imm0 = cpu.immediate_{}();",
                self.param_type
            );
        }

        Ok(())
    }

    fn from_operand(operand: &str) -> Self {
        if operand.starts_with("out ") {
            let inner: String = operand.chars().skip(4).collect();
            let mut param = Self::from_operand(&inner);
            param.output = OutputMode::Out;
            return param;
        }
        if operand.starts_with("inout ") {
            let inner: String = operand.chars().skip(6).collect();
            let mut param = Self::from_operand(&inner);
            param.output = OutputMode::Inout;
            return param;
        } else if operand.starts_with("(") {
            let len = operand.chars().count();
            let inner: String = operand.chars().skip(1).take(len - 2).collect();
            let leaf_node = Self::from_operand(&inner);

            let mut offset_node = leaf_node.clone();
            // when the inner parameter is 8-bit, we need to add it to 0xff00.
            // for reasons.
            match offset_node.param_type {
                ParameterType::I8 | ParameterType::U8 => {
                    offset_node.fullcode =
                        String::new() + "(" + &leaf_node.fullcode + " as u32 + 0xff00) as u16"
                }
                _ => {}
            };
            offset_node.inner = Some(Box::new(leaf_node));

            return Parameter {
                fullcode: format!("cpu.address({})", offset_node.fullcode),
                hl: None,
                inner: Some(Box::new(offset_node)),
                param_type: ParameterType::U8,
                output: OutputMode::None,
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
            return Self::from_operand("r8");
        } else if operand == "d8" {
            return Parameter::immediate(ParameterType::U8);
        } else if operand == "d16" {
            return Parameter::immediate(ParameterType::U16);
        } else if operand == "a8" {
            return Parameter::immediate(ParameterType::U8);
        } else if operand == "a16" {
            return Parameter::immediate(ParameterType::U16);
        } else if operand == "r8" {
            return Parameter::immediate(ParameterType::I8);
        } else if let Ok(num) = operand.parse::<usize>() {
            return Parameter::new(format!("{}", num), ParameterType::U8);
        } else if operand.ends_with("H") && operand.len() == 3 {
            let num: String = operand.chars().take(2).collect();
            return Parameter::new(format!("0x{}", num), ParameterType::U16);
        } else if operand == "A" {
            return Parameter::new(String::from("cpu.AF.r8.first"), ParameterType::U8);
        } else if operand == "F" {
            return Parameter::new(String::from("cpu.AF.r8.second"), ParameterType::U8);
        } else if operand == "B" {
            return Parameter::new(String::from("cpu.BC.r8.first"), ParameterType::U8);
        } else if operand == "C" {
            return Parameter::new(String::from("cpu.BC.r8.second"), ParameterType::U8);
        } else if operand == "D" {
            return Parameter::new(String::from("cpu.DE.r8.first"), ParameterType::U8);
        } else if operand == "E" {
            return Parameter::new(String::from("cpu.DE.r8.second"), ParameterType::U8);
        } else if operand == "H" {
            return Parameter::new(String::from("cpu.HL.r8.first"), ParameterType::U8);
        } else if operand == "L" {
            return Parameter::new(String::from("cpu.HL.r8.second"), ParameterType::U8);
        } else if operand == "L" {
            return Parameter::new(String::from("cpu.HL.r8.second"), ParameterType::U8);
        } else if operand == "Z" {
            return Parameter::new(String::from("cpu.z()"), ParameterType::Bool);
        } else if operand == "NZ" {
            return Parameter::new(String::from("!cpu.z()"), ParameterType::Bool);
        } else if operand == "C" || operand == "c" {
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

    fn write_for_post(&self, outfile: &mut File, input: Option<&Parameter>) -> std::io::Result<()> {
        if let Some(ref inner) = self.inner {
            writeln!(outfile, "\t\t\tlet addr = {};", inner.fullcode)?;

            let mut param_type = ParameterType::U8;
            if let Some(ref i) = input {
                param_type = i.param_type.clone();
            }

            match param_type {
                ParameterType::U16 => writeln!(outfile, "\t\t\tcpu.set_address16(addr, out);")?,
                _ => writeln!(outfile, "\t\t\tcpu.set_address(addr, out);")?,
            }
        } else {
            writeln!(outfile, "\t\t\t{} = out;", self.fullcode)?;
        }

        Ok(())
    }

    fn hl_mode(&self) -> Option<HLMagic> {
        if self.hl.is_some() {
            return self.hl;
        }
        if let Some(ref inner) = self.inner {
            return inner.hl_mode();
        }
        None
    }
}

struct FunctionDesc {
    name: String,
    inputs: Vec<Parameter>,
    output: Option<Parameter>,
}

impl FunctionDesc {
    pub fn from_opcode(opcode: &OpCodeDesc) -> Self {
        let mut name = String::new();
        let mut inputs = vec![];
        let mut output: Option<Parameter> = None;

        for operand in &opcode.operands {
            let param = Parameter::from_operand(operand);

            match param.output {
                OutputMode::None => inputs.push(param),
                OutputMode::Out => {
                    assert!(output.is_none(), "This function has 2 outputs. NANI????");
                    output = Some(param);
                }
                OutputMode::Inout => {
                    assert!(output.is_none(), "This function has 2 outputs. NANI????");
                    output = Some(param.clone());
                    inputs.push(param);
                }
            }
        }

        name += &opcode.mnemonic;
        name += &maybe_flag("z", &opcode.flagsZNHC[0]);
        name += &maybe_flag("n", &opcode.flagsZNHC[1]);
        name += &maybe_flag("h", &opcode.flagsZNHC[2]);
        name += &maybe_flag("c", &opcode.flagsZNHC[3]);

        //append the types (including outputs) to do overloading
        for parameter in &inputs {
            name += &format!("_{}", parameter.param_type);
        }
        if let Some(parameter) = &output {
            name += &format!("_out_{}", parameter.param_type);
        }

        FunctionDesc {
            name: name,
            inputs: inputs,
            output: output,
        }
    }

    fn write_pre(&self, outfile: &mut File) -> std::io::Result<()> {
        //load the immediate first to use the old PC value
        for input in &self.inputs {
            input.write_immediate_load(outfile)?;
        }
        if let Some(out) = &self.output {
            out.write_immediate_load(outfile)?;
        }

        //get the parameters in their own line to help borrowck not confuse itself and die
        for idx in 0..self.inputs.len() {
            &writeln!(
                outfile,
                "\t\t\tlet reg{} = {};",
                idx, self.inputs[idx].fullcode
            )?;
        }
        if let Some(_) = &self.output {
            &writeln!(outfile, "\t\t\tlet mut out;")?;
        }
        Ok(())
    }

    fn write_post(&self, outfile: &mut File) -> std::io::Result<()> {
        if let Some(parameter) = &self.output {
            if self.inputs.len() > 0 {
                parameter.write_for_post(outfile, Some(&self.inputs[0]))?;
            } else {
                parameter.write_for_post(outfile, None)?;
            }
        }
        Ok(())
    }

    fn hl_mode(&self) -> Option<HLMagic> {
        for param in &self.inputs {
            if let Some(hl) = param.hl_mode() {
                return Some(hl);
            }
        }
        if let Some(ref param) = self.output {
            if let Some(hl) = param.hl_mode() {
                return Some(hl);
            }
        }
        None
    }
}

const HEADER: &str = r#"
use cpu::*;
use bit_field::BitField;

#[allow(unused, unreachable_code)]
pub unsafe fn interpret(instruction: u16, cpu: &mut CPU) {
    match instruction {
"#;

const FOOTER: &str = r#"
        _ => panic!("instruction not known")
    }
}"#;

fn write_opcodes(
    outfile: &mut File,
    opcodes: &BTreeMap<String, OpCodeDesc>,
    codes: &FunctionCodeMap,
) -> std::io::Result<Vec<FunctionDesc>> {
    let mut function_list = vec![];

    for (name, opcode) in opcodes {
        writeln!(outfile, "\t\t{} => {{", name)?;

        let function = FunctionDesc::from_opcode(opcode);

        let not_found = FunctionCode::not_found(&function.name);
        let code = codes.get(&function.name).unwrap_or(&not_found);

        writeln!(outfile, "\t\t\t// {}", function.name)?;

        function.write_pre(outfile)?;

        //things with a prefix need to be shorter because they're counting the prefix
        let mut bytes = opcode.bytes;
        if name.len() > 4 {
            bytes -= 1;
        }
        writeln!(outfile, "\t\t\tcpu.PC += {};", bytes)?;

        for line in &code.lines {
            writeln!(outfile, "\t{}", line)?;
        }

        function.write_post(outfile)?;

        //do the HL magic
        if let Some(hl) = &function.hl_mode() {
            match hl {
                HLMagic::Inc => {
                    writeln!(
                        outfile,
                        "\t\t\t{0} = {0}.wrapping_add(1);",
                        Parameter::from_operand("HL").fullcode
                    )?;
                }
                HLMagic::Dec => {
                    writeln!(
                        outfile,
                        "\t\t\t{0} = {0}.wrapping_sub(1);",
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

        //cycle
        writeln!(outfile, "\t\t\tcpu.run_cycles({});", opcode.cycles)?;

        writeln!(outfile, "\t\t}},")?;
    }
    Ok(function_list)
}

fn write_interpreter(
    opcodes: &BTreeMap<String, OpCodeDesc>,
    codes: &FunctionCodeMap,
) -> std::io::Result<Vec<FunctionDesc>> {
    let outfile = &mut File::create(INTERPRETER_PATH)?;

    writeln!(outfile, "{}", HEADER)?;

    let function_list = write_opcodes(outfile, opcodes, codes)?;

    writeln!(outfile, "{}", FOOTER)?;

    Ok(function_list)
}

fn write_function_stub(
    outfile: &mut File,
    function: &FunctionDesc,
    code: &FunctionCode,
) -> std::io::Result<()> {
    //compose the parameters
    writeln!(outfile, "\t{{")?;
    writeln!(outfile, "\t// NAME: {}", function.name)?;
    function.write_pre(outfile)?;

    writeln!(outfile, "\t//----------------")?;
    for line in &code.lines {
        writeln!(outfile, "{}", line)?;
    }
    writeln!(outfile, "\t//----------------")?;
    function.write_post(outfile)?;
    writeln!(outfile, "\t}}")?;

    Ok(())
}

const STUBS_PATH: &str = "src/function_stubs.rs";
const INTERPRETER_PATH: &str = "src/interpreter.rs";
const OPCODES_PATH: &str = "opcodes.json";

fn parse_function_stubs() -> std::io::Result<FunctionCodeMap> {
    let name_regex = Regex::new(".*// NAME: (.*)").unwrap();

    let mut stubs = BTreeMap::new();

    let file = BufReader::new(File::open(STUBS_PATH)?);

    let mut autogen = true;
    let mut name = String::new();

    for line_err in file.lines() {
        let line = line_err?;
        if line.contains("//----------------") {
            autogen = !autogen;
            continue;
        }

        if let Some(new_name) = name_regex.captures(&line) {
            name = new_name.get(1).unwrap().as_str().to_owned();

            stubs.insert(name.clone(), FunctionCode::new());
        }

        if !autogen {
            stubs.get_mut(&name).unwrap().lines.push(line);
        }
    }

    Ok(stubs)
}

fn write_function_stubs(
    functions: &[FunctionDesc],
    already_defined: &FunctionCodeMap,
) -> std::io::Result<()> {
    //write out stubs for the functions that were found
    let mut dedupd = BTreeMap::new();

    for func in functions {
        dedupd.insert(func.name.clone(), func);
    }

    //open the output file
    let outfile = &mut File::create(STUBS_PATH)?;

    write!(
        outfile,
        r#"
use cpu::*;
use bit_field::BitField;

#[allow(unused, unreachable_code)]
unsafe fn stubs(cpu: &mut CPU) {{
"#
    )?;

    //write out all the remaining functions in alphabetical order
    for func in dedupd.values() {
        write_function_stub(
            outfile,
            func,
            already_defined
                .get(&func.name)
                .unwrap_or(&FunctionCode::not_found(&func.name)),
        )?;
    }
    writeln!(
        outfile,
        r#"
}}"#
    )?;

    Ok(())
}

fn main() {
    //things for cargo
    println!("cargo:rerun-if-changed={}", OPCODES_PATH);
    println!("cargo:rerun-if-changed={}", STUBS_PATH);
    println!("cargo:rerun-if-changed={}", INTERPRETER_PATH);

    let codes = &parse_function_stubs().unwrap();

    let opcodes: BTreeMap<String, OpCodeDesc> =
        serde_json::from_reader(File::open(OPCODES_PATH).unwrap()).unwrap();

    let functions = write_interpreter(&opcodes, codes).unwrap();

    write_function_stubs(&functions, codes).unwrap();
}
