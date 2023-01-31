use crate::{
    ast::{Expr, ExprArgs, ExprKind},
    common::{parse_u16, TokenEnum},
    instructions::Instructions,
    parse::{InstructionParser, ParserHelper},
};

#[derive(Debug)]
struct CodeGen;

impl CodeGen {
    fn new() -> Self {
        CodeGen
    }

    pub fn generate(&mut self, input: &Vec<Expr>) -> Vec<u8> {
        input
            .iter()
            .map(|expr| self.gen_expr(expr))
            .flatten()
            .collect()
    }

    pub fn gen_expr(&mut self, expr: &Expr) -> Vec<u8> {
        match expr.kind {
            ExprKind::Mov => self.gen_mov(&expr.args),
            ExprKind::Add => self.gen_add(&expr.args),
            ExprKind::Sub => self.gen_sub(&expr.args),
            ExprKind::Mul => self.gen_mul(&expr.args),
            ExprKind::Inc => self.gen_inc(&expr.args),
            ExprKind::Dec => self.gen_dec(&expr.args),

            ExprKind::Lsf => self.gen_left_shift(&expr.args),
            ExprKind::Rsf => self.gen_rigth_shift(&expr.args),
            ExprKind::And => self.gen_and(&expr.args),
            ExprKind::Or => self.gen_or(&expr.args),
            ExprKind::Xor => self.gen_xor(&expr.args),
            ExprKind::JmpEQ => self.gen_jmp_eq(&expr.args),
            ExprKind::JmpGT => self.gen_jmp_gt(&expr.args),
            ExprKind::JmpNotEQ => self.gen_jmp_not_eq(&expr.args),
            ExprKind::JmpLT => self.gen_jmp_lt(&expr.args),

            ExprKind::Push => self.gen_push(&expr.args),
            ExprKind::Pop => self.gen_pop(&expr.args),
            ExprKind::HLT => vec![Instructions::HLT as u8],
            ExprKind::Call => self.gen_call(&expr.args),
            ExprKind::Ret => vec![Instructions::RET as u8],
            _ => unimplemented!("Add more instructions!"),
        }
    }

    // XXX: write macros

    pub fn gen_mov(&mut self, args: &ExprArgs) -> Vec<u8> {
        match args {
            ExprArgs::Double(TokenEnum::Lit(lit), TokenEnum::Ident(mb_reg)) => {
                // XXX: very wrong but for now ok
                let reg = ParserHelper::parse_reg(mb_reg).unwrap();
                let (lit_h, lit_l) = parse_u16(lit);
                vec![Instructions::MOV_LIT_REG as u8, lit_h, lit_l, reg as u8]
            }
            ExprArgs::Double(TokenEnum::Ident(mb_reg1), TokenEnum::Ident(mb_reg2)) => {
                let reg1 = ParserHelper::parse_reg(mb_reg1).unwrap();
                let reg2 = ParserHelper::parse_reg(mb_reg2).unwrap();
                vec![Instructions::MOV_REG_REG as u8, reg1 as u8, reg2 as u8]
            }
            ExprArgs::Double(TokenEnum::Ident(mb_reg), TokenEnum::Mem(mem)) => {
                let reg = ParserHelper::parse_reg(mb_reg).unwrap();
                let (mem_h, mem_l) = parse_u16(mem);
                vec![Instructions::MOV_REG_MEM as u8, reg as u8, mem_h, mem_l]
            }
            ExprArgs::Double(TokenEnum::Mem(mem), TokenEnum::Ident(mb_reg)) => {
                let reg = ParserHelper::parse_reg(mb_reg).unwrap();
                let (mem_h, mem_l) = parse_u16(mem);
                vec![Instructions::MOV_MEM_REG as u8, mem_h, mem_l, reg as u8]
            }
            ExprArgs::Double(TokenEnum::Lit(lit), TokenEnum::Mem(mem)) => {
                let (lit_h, lit_l) = parse_u16(lit);
                let (mem_h, mem_l) = parse_u16(mem);
                vec![Instructions::MOV_LIT_MEM as u8, lit_h, lit_l, mem_h, mem_l]
            }
            ExprArgs::Double(TokenEnum::Ref(mb_regref), TokenEnum::Ident(mb_reg)) => {
                let regref = ParserHelper::parse_reg(mb_regref).unwrap();
                let reg = ParserHelper::parse_reg(mb_reg).unwrap();
                vec![Instructions::MOV_REG_PTR_REG as u8, regref as u8, reg as u8]
            }
            ExprArgs::Triple(
                TokenEnum::Mem(mem),
                TokenEnum::Ident(mb_reg1),
                TokenEnum::Ident(mb_reg2),
            ) => {
                let reg1 = ParserHelper::parse_reg(mb_reg1).unwrap();
                let reg2 = ParserHelper::parse_reg(mb_reg2).unwrap();
                let (mem_h, mem_l) = parse_u16(mem);
                vec![
                    Instructions::MOV_LIT_OFF_REG as u8,
                    mem_h,
                    mem_l,
                    reg1 as u8,
                    reg2 as u8,
                ]
            }
            _ => panic!("mov {:?} not implemented", args),
        }
    }

    fn gen_add(&self, args: &ExprArgs) -> Vec<u8> {
        match args {
            ExprArgs::Double(TokenEnum::Ident(mb_reg1), TokenEnum::Ident(mb_reg2)) => {
                let reg1 = ParserHelper::parse_reg(mb_reg1).unwrap();
                let reg2 = ParserHelper::parse_reg(mb_reg2).unwrap();
                vec![Instructions::ADD_REG_REG as u8, reg1 as u8, reg2 as u8]
            }
            ExprArgs::Double(TokenEnum::Lit(lit), TokenEnum::Ident(mb_reg)) => {
                let reg = ParserHelper::parse_reg(mb_reg).unwrap();
                let (lit_h, lit_l) = parse_u16(lit);
                vec![Instructions::ADD_LIT_REG as u8, lit_h, lit_l, reg as u8]
            }
            _ => panic!("add {:?} not implemented", args),
        }
    }

    fn gen_sub(&self, args: &ExprArgs) -> Vec<u8> {
        match args {
            ExprArgs::Double(TokenEnum::Ident(mb_reg1), TokenEnum::Ident(mb_reg2)) => {
                let reg1 = ParserHelper::parse_reg(mb_reg1).unwrap();
                let reg2 = ParserHelper::parse_reg(mb_reg2).unwrap();
                vec![Instructions::SUB_REG_REG as u8, reg1 as u8, reg2 as u8]
            }
            ExprArgs::Double(TokenEnum::Lit(lit), TokenEnum::Ident(mb_reg)) => {
                let reg = ParserHelper::parse_reg(mb_reg).unwrap();
                let (lit_h, lit_l) = parse_u16(lit);
                vec![Instructions::SUB_LIT_REG as u8, lit_h, lit_l, reg as u8]
            }
            ExprArgs::Double(TokenEnum::Ident(mb_reg), TokenEnum::Lit(lit)) => {
                let reg = ParserHelper::parse_reg(mb_reg).unwrap();
                let (lit_h, lit_l) = parse_u16(lit);
                vec![Instructions::SUB_REG_LIT as u8, reg as u8, lit_h, lit_l]
            }
            _ => panic!("sub {:?} not implemented", args),
        }
    }

    fn gen_mul(&self, args: &ExprArgs) -> Vec<u8> {
        match args {
            ExprArgs::Double(TokenEnum::Ident(mb_reg1), TokenEnum::Ident(mb_reg2)) => {
                let reg1 = ParserHelper::parse_reg(mb_reg1).unwrap();
                let reg2 = ParserHelper::parse_reg(mb_reg2).unwrap();
                vec![Instructions::MUL_REG_REG as u8, reg1 as u8, reg2 as u8]
            }
            ExprArgs::Double(TokenEnum::Lit(lit), TokenEnum::Ident(mb_reg)) => {
                let reg = ParserHelper::parse_reg(mb_reg).unwrap();
                let (lit_h, lit_l) = parse_u16(lit);
                vec![Instructions::MUL_LIT_REG as u8, lit_h, lit_l, reg as u8]
            }
            _ => panic!("mul {:?} not implemented", args),
        }
    }

    fn gen_push(&self, args: &ExprArgs) -> Vec<u8> {
        match args {
            ExprArgs::Single(TokenEnum::Ident(mb_reg)) => {
                let reg = ParserHelper::parse_reg(mb_reg).unwrap();
                vec![Instructions::PSH_REG as u8, reg as u8]
            }
            ExprArgs::Single(TokenEnum::Lit(lit)) => {
                let (lit_h, lit_l) = parse_u16(lit);
                vec![Instructions::PSH_LIT as u8, lit_h, lit_l]
            }
            _ => panic!("push {:?} not implemented", args),
        }
    }

    fn gen_pop(&self, args: &ExprArgs) -> Vec<u8> {
        match args {
            ExprArgs::Single(TokenEnum::Ident(mb_reg)) => {
                let reg = ParserHelper::parse_reg(mb_reg).unwrap();
                vec![Instructions::POP as u8, reg as u8]
            }
            _ => panic!("pop {:?} not implemented", args),
        }
    }

    fn gen_call(&self, args: &ExprArgs) -> Vec<u8> {
        match args {
            ExprArgs::Single(TokenEnum::Ident(mb_reg)) => {
                let reg = ParserHelper::parse_reg(mb_reg).unwrap();
                vec![Instructions::CALL_REG as u8, reg as u8]
            }
            ExprArgs::Single(TokenEnum::Lit(lit)) => {
                let (lit_h, lit_l) = parse_u16(lit);
                vec![Instructions::CALL_LIT as u8, lit_h, lit_l]
            }
            _ => panic!("call {:?} not implemented", args),
        }
    }

    fn gen_inc(&self, args: &ExprArgs) -> Vec<u8> {
        match args {
            ExprArgs::Single(TokenEnum::Ident(mb_reg)) => {
                let reg = ParserHelper::parse_reg(mb_reg).unwrap();
                vec![Instructions::INC_REG as u8, reg as u8]
            }
            _ => panic!("inc {:?} not implemented", args),
        }
    }

    fn gen_dec(&self, args: &ExprArgs) -> Vec<u8> {
        match args {
            ExprArgs::Single(TokenEnum::Ident(mb_reg)) => {
                let reg = ParserHelper::parse_reg(mb_reg).unwrap();
                vec![Instructions::DEC_REG as u8, reg as u8]
            }
            _ => panic!("dec {:?} not implemented", args),
        }
    }

    fn gen_and(&self, args: &ExprArgs) -> Vec<u8> {
        todo!()
    }

    fn gen_or(&self, args: &ExprArgs) -> Vec<u8> {
        todo!()
    }

    fn gen_xor(&self, args: &ExprArgs) -> Vec<u8> {
        todo!()
    }

    fn gen_jmp_eq(&self, args: &ExprArgs) -> Vec<u8> {
        todo!()
    }

    fn gen_jmp_gt(&self, args: &ExprArgs) -> Vec<u8> {
        todo!()
    }

    fn gen_jmp_not_eq(&self, args: &ExprArgs) -> Vec<u8> {
        todo!()
    }

    fn gen_jmp_lt(&self, args: &ExprArgs) -> Vec<u8> {
        todo!()
    }

    fn gen_left_shift(&self, args: &ExprArgs) -> Vec<u8> {
        todo!()
    }

    fn gen_rigth_shift(&self, args: &ExprArgs) -> Vec<u8> {
        todo!()
    }
}

#[test]
fn codegen_mov() {
    let mut parser = InstructionParser::new();
    let mut codegen = CodeGen::new();

    let parsed = parser.parse("mov $1, r2").unwrap();
    let generated = codegen.generate(&parsed);
    assert_eq!(generated, vec![0x10u8, 0x00u8, 0x01u8, 0x03u8]);

    let parsed = parser.parse("mov &r1, r2").unwrap();
    let generated = codegen.generate(&parsed);
    assert_eq!(generated, vec![0x1Cu8, 0x02u8, 0x03u8]);

    let parsed = parser
        .parse(
            "mov $1, r1
        mov &2, r2
        mov r1, r3",
        )
        .unwrap();
    let generated = codegen.generate(&parsed);
    assert_eq!(
        generated,
        vec![
            0x10u8, 0x00u8, 0x01u8, 0x02u8, 0x13u8, 0x00u8, 0x02u8, 0x03u8, 0x11u8, 0x02u8, 0x04u8
        ]
    );
}

#[test]
fn codegen_add() {
    let mut parser = InstructionParser::new();
    let mut codegen = CodeGen::new();

    let parsed = parser.parse("add $1, r2").unwrap();
    let generated = codegen.generate(&parsed);
    assert_eq!(generated, vec![0x3Fu8, 0x00u8, 0x01u8, 0x03u8]);

    let parsed = parser.parse("add r1, r2").unwrap();
    let generated = codegen.generate(&parsed);
    assert_eq!(generated, vec![0x14u8, 0x02u8, 0x03u8]);
}

#[test]
fn codegen_sub() {
    let mut parser = InstructionParser::new();
    let mut codegen = CodeGen::new();

    let parsed = parser.parse("sub $1, r2").unwrap();
    let generated = codegen.generate(&parsed);
    assert_eq!(generated, vec![0x16u8, 0x00u8, 0x01u8, 0x03u8]);

    let parsed = parser.parse("sub r1, r2").unwrap();
    let generated = codegen.generate(&parsed);
    assert_eq!(generated, vec![0x1Fu8, 0x02u8, 0x03u8]);

    let parsed = parser.parse("sub r1, $1").unwrap();
    let generated = codegen.generate(&parsed);
    assert_eq!(generated, vec![0x1Eu8, 0x02u8, 0x00u8, 0x01u8]);
}

#[test]
fn codegen_mul() {
    let mut parser = InstructionParser::new();
    let mut codegen = CodeGen::new();

    let parsed = parser.parse("mul $1, r2").unwrap();
    let generated = codegen.generate(&parsed);
    assert_eq!(generated, vec![0x20u8, 0x00u8, 0x01u8, 0x03u8]);

    let parsed = parser.parse("mul r1, r2").unwrap();
    let generated = codegen.generate(&parsed);
    assert_eq!(generated, vec![0x21u8, 0x02u8, 0x03u8]);
}

#[test]
fn codegen_stack() {
    let mut parser = InstructionParser::new();
    let mut codegen = CodeGen::new();

    let parsed = parser.parse("push $1").unwrap();
    let generated = codegen.generate(&parsed);
    assert_eq!(generated, vec![0x17u8, 0x00u8, 0x01u8]);

    let parsed = parser.parse("push r1").unwrap();
    let generated = codegen.generate(&parsed);
    assert_eq!(generated, vec![0x18u8, 0x02u8]);

    let parsed = parser.parse("pop r2").unwrap();
    let generated = codegen.generate(&parsed);
    assert_eq!(generated, vec![0x1Au8, 0x03u8]);
}

#[test]
fn codegen_subroutine() {
    let mut parser = InstructionParser::new();
    let mut codegen = CodeGen::new();

    let parsed = parser.parse("call $1").unwrap();
    let generated = codegen.generate(&parsed);
    assert_eq!(generated, vec![0x5Eu8, 0x00u8, 0x01u8]);

    let parsed = parser.parse("call r1").unwrap();
    let generated = codegen.generate(&parsed);
    assert_eq!(generated, vec![0x5Fu8, 0x02u8]);

    let parsed = parser.parse("ret").unwrap();
    let generated = codegen.generate(&parsed);
    assert_eq!(generated, vec![0x60u8]);
}

#[test]
fn codegen_hlt() {
    let mut parser = InstructionParser::new();
    let mut codegen = CodeGen::new();

    let parsed = parser.parse("hlt").unwrap();
    let generated = codegen.generate(&parsed);
    assert_eq!(generated, vec![0xFFu8]);
}

#[test]
fn codegen_inc_dec() {
    let mut parser = InstructionParser::new();
    let mut codegen = CodeGen::new();

    let parsed = parser.parse("inc r1").unwrap();
    let generated = codegen.generate(&parsed);
    assert_eq!(generated, vec![0x35u8, 0x02u8]);

    let parsed = parser.parse("dec r1").unwrap();
    let generated = codegen.generate(&parsed);
    assert_eq!(generated, vec![0x36u8, 0x02u8]);
}
