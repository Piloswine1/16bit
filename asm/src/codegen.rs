use crate::common::*;
use crate::instructions::Instructions;

pub struct CodeGen;

#[derive(Debug)]
pub enum CodeGenErr {
    UnsupportedInstruction(LVal, RVal),
}
type CodeGenRes = Result<Vec<u8>, CodeGenErr>;

impl CodeGen {
    pub fn new() -> Self {
        CodeGen
    }

    fn parse_mov(&self, lval: &LVal, rval: &RVal) -> CodeGenRes {
        match (lval, rval) {
            (LVal::Hex(hex), RVal::Reg(reg)) => {
                let mut res = vec![Instructions::MOV_LIT_REG as u8];
                res.extend_from_slice(parse_u16(hex).as_slice());
                res.push(*reg as u8);
                Ok(res.to_vec())
            }
            (LVal::Hex(hex), RVal::Mem(mem)) => {
                let mut res = vec![Instructions::MOV_LIT_MEM as u8];
                res.extend_from_slice(parse_u16(hex).as_slice());
                res.extend_from_slice(parse_u16(mem).as_slice());
                Ok(res.to_vec())
            }
            (LVal::Reg(regl), RVal::Reg(regr)) => {
                let mut res = vec![Instructions::MOV_REG_REG as u8];
                res.push(*regl as u8);
                res.push(*regr as u8);
                Ok(res.to_vec())
            }
            (LVal::Reg(reg), RVal::Mem(mem)) => {
                let mut res = vec![Instructions::MOV_REG_MEM as u8];
                res.push(*reg as u8);
                res.extend_from_slice(parse_u16(mem).as_slice());
                Ok(res)
            }
            (LVal::Mem(mem), RVal::Reg(reg)) => {
                let mut res = vec![Instructions::MOV_MEM_REG as u8];
                res.extend_from_slice(parse_u16(mem).as_slice());
                res.push(*reg as u8);
                Ok(res)
            }
            _ => Err(CodeGenErr::UnsupportedInstruction(
                lval.to_owned(),
                rval.to_owned(),
            )),
        }
    }

    pub fn code_gen(&self, commands: &[ASMCommand]) -> CodeGenRes {
        let mut res: Vec<u8> = Vec::new();
        for command in commands {
            match command {
                ASMCommand::Mov(lval, rval) => {
                    let parsed = self.parse_mov(lval, rval)?;
                    res = [res, parsed].concat();
                }
                ASMCommand::Hlt => {
                    res.push(Instructions::HLT as u8);
                }
            }
        }
        return Ok(res);
    }
}
