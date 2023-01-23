pub mod codegen;
pub mod common;
pub mod instructions;
pub mod parse;

fn main() {}

#[test]
fn code_gen_test() {
    use codegen::CodeGen;
    use common::*;
    use instructions::Instructions;

    let mut code_gen = CodeGen::new();
    let res = code_gen.code_gen(&[ASMCommand::Mov(LVal::Hex(0x12u16), RVal::Reg(Regs::R1))]);

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

#[test]
fn parse() {
    use common::*;
    use parse::*;

    let parser = Parser::new();
    let parsed = parser.parse("mov $12 r1");

    assert_eq!(
        parsed.unwrap(),
        [ASMCommand::Mov(LVal::Hex(0x12u16), RVal::Reg(Regs::R1))]
    );
}

#[test]
fn integrational() {
    use codegen::CodeGen;
    use instructions::Instructions;
    use common::*;
    use parse::*;

    let parser = Parser::new();
    let mut code_gen = CodeGen::new();
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
