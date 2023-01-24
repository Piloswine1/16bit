pub mod codegen;
pub mod common;
pub mod instructions;
pub mod parse;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::PathBuf,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    input: PathBuf,
    /// Dump ast
    #[arg(short, long, default_value_t = false)]
    dump: bool,
    /// Output file name
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    use codegen::CodeGen;
    use parse::Parser;

    let args = Args::parse();
    let parser = Parser::new();
    let codegen = CodeGen::new();

    let file = File::open(args.input)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let parsed = parser.parse(contents.as_str()).unwrap();

    if args.dump {
        println!("{:?}", parsed);
        return Ok(());
    }

    let binary = codegen.code_gen(parsed.as_slice()).unwrap();

    let out_name = args.output.unwrap_or(PathBuf::from("a.out"));
    let mut out_file = File::create(&out_name)?;
    out_file.write_all(binary.as_slice())?;

    println!("Wrote {} bytes to {}", binary.len(), out_name.display());

    Ok(())
}

#[test]
fn code_gen_test() {
    use codegen::CodeGen;
    use common::*;
    use instructions::Instructions;

    let code_gen = CodeGen::new();
    let res = code_gen.code_gen(&[
        ASMCommand::Mov(LVal::Hex(0x12u16), RVal::Reg(Regs::R1)),
        ASMCommand::Hlt,
    ]);

    let res_unwrapped = res.unwrap();
    assert_eq!(
        res_unwrapped,
        [
            Instructions::MOV_LIT_REG as u8,
            0x00u8,
            0x12u8,
            Regs::R1 as u8,
            Instructions::HLT as u8,
        ]
    );
    assert_eq!(res_unwrapped, [0x10u8, 0x00u8, 0x12u8, 0x02u8, 0xffu8])
}

#[test]
fn parse() {
    use common::*;
    use parse::*;

    let parser = Parser::new();
    let parsed = parser.parse("mov $12 r1\n hlt");

    assert_eq!(
        parsed.unwrap(),
        [ASMCommand::Mov(LVal::Hex(0x12u16), RVal::Reg(Regs::R1)), ASMCommand::Hlt]
    );
}

#[test]
fn integrational() {
    use codegen::CodeGen;
    use common::*;
    use instructions::Instructions;
    use parse::*;

    let parser = Parser::new();
    let code_gen = CodeGen::new();
    let parsed = parser.parse("mov $12 r1").unwrap();

    assert_eq!(
        parsed,
        [ASMCommand::Mov(LVal::Hex(0x12u16), RVal::Reg(Regs::R1))]
    );

    let res = code_gen.code_gen(parsed.as_slice());

    let res_unwrapped = res.unwrap();
    assert_eq!(
        res_unwrapped,
        [
            Instructions::MOV_LIT_REG as u8,
            0x00u8,
            0x12u8,
            Regs::R1 as u8
        ]
    );
    assert_eq!(res_unwrapped, [0x10u8, 0x00u8, 0x12u8, 0x02u8])
}
