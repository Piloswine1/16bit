use crate::{
    ast::{Expr, ExprArgs, ExprKind},
    common::{parse_u16, TokenEnum},
    instructions::Instructions,
    parse::{InstructionParser, ParserHelper},
};

macro_rules! gen_hand {
    (ref $i:ident) => {
        TokenEnum::Ref($i)
    };
    (mem $i:ident) => {
        TokenEnum::Mem($i)
    };
    (reg $i:ident) => {
        TokenEnum::Ident($i)
    };
    (lit $i:ident) => {
        TokenEnum::Lit($i)
    };
}

macro_rules! gen_body {
    (reg $r:ident $res:ident) => {
        let $r = ParserHelper::parse_reg($r).unwrap();
        $res.push($r as u8);
    };
    (ref $($tail:tt)*) => {gen_body!(reg $($tail)*)};
    (mem $($tail:tt)*) => {gen_body!(u16 $($tail)*)};
    (lit $($tail:tt)*) => {gen_body!(u16 $($tail)*)};
    (u16 $i:ident $res:ident) => {
        {
            let (h, l) = parse_u16($i);
            $res.push(h);
            $res.push(l);
        }
    };
}

macro_rules! gen_patt {
    (
        $args:ident:
        $(1 $i1:ident($arg11:ident);)*
        $(2 $i2:ident($arg21:ident, $arg22:ident);)*
        $(3 $i3:ident($arg31:ident, $arg32:ident, $arg33:ident);)*
    ) => {
        match $args {
            $(
                ExprArgs::Single(gen_hand!($arg11 arg)) => {
                    let mut res = vec![Instructions::$i1 as u8];
                    gen_body!($arg11 arg res);
                    res
                }
            )*
            $(
                ExprArgs::Double(gen_hand!($arg21 arg1), gen_hand!($arg22 arg2)) => {
                    let mut res = vec![Instructions::$i2 as u8];
                    gen_body!($arg21 arg1 res);
                    gen_body!($arg22 arg2 res);
                    res
                }
            )*
            $(
                ExprArgs::Triple(gen_hand!($arg31 arg1), gen_hand!($arg32 arg2), gen_hand!($arg33 arg3)) => {
                    let mut res = vec![Instructions::$i3 as u8];
                    gen_body!($arg31 arg1 res);
                    gen_body!($arg32 arg2 res);
                    gen_body!($arg33 arg3 res);
                    res
                }
            )*
            _ => panic!("{:?} not implemented", $args),
        }
    };
}

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

    pub fn gen_mov(&mut self, args: &ExprArgs) -> Vec<u8> {
        gen_patt!(
            args:
            2 MOV_LIT_REG(lit, reg);
            2 MOV_REG_REG(reg, reg);
            2 MOV_REG_MEM(reg, mem);
            2 MOV_MEM_REG(mem, reg);
            2 MOV_LIT_MEM(lit, mem);
            2 MOV_REG_PTR_REG(ref, reg);
            3 MOV_LIT_OFF_REG(mem, reg, reg);
        )
    }

    fn gen_add(&self, args: &ExprArgs) -> Vec<u8> {
        gen_patt!(
            args:
            2 ADD_REG_REG(reg, reg);
            2 ADD_LIT_REG(lit, reg);
        )
    }

    fn gen_sub(&self, args: &ExprArgs) -> Vec<u8> {
        gen_patt!(
            args:
            2 SUB_REG_REG(reg, reg);
            2 SUB_LIT_REG(lit, reg);
            2 SUB_REG_LIT(reg, lit);
        )
    }

    fn gen_mul(&self, args: &ExprArgs) -> Vec<u8> {
        gen_patt!(
            args:
            2 MUL_REG_REG(reg, reg);
            2 MUL_LIT_REG(lit, reg);
        )
    }

    fn gen_push(&self, args: &ExprArgs) -> Vec<u8> {
        gen_patt!(
            args:
            1 PSH_REG(reg);
            1 PSH_LIT(lit);
        )
    }

    fn gen_pop(&self, args: &ExprArgs) -> Vec<u8> {
        gen_patt!(
            args:
            1 POP(reg);
        )
    }

    fn gen_call(&self, args: &ExprArgs) -> Vec<u8> {
        gen_patt!(
            args:
            1 CALL_REG(reg);
            1 CALL_LIT(lit);
        )
    }

    fn gen_inc(&self, args: &ExprArgs) -> Vec<u8> {
        gen_patt!(
            args:
            1 INC_REG(reg);
        )
    }

    fn gen_dec(&self, args: &ExprArgs) -> Vec<u8> {
        gen_patt!(
            args:
            1 DEC_REG(reg);
        )
    }

    fn gen_and(&self, args: &ExprArgs) -> Vec<u8> {
        gen_patt!(
            args:
            2 AND_REG_REG(reg, reg);
            2 AND_REG_LIT(reg, lit);
        )
    }

    fn gen_or(&self, args: &ExprArgs) -> Vec<u8> {
        gen_patt!(
            args:
            2 OR_REG_REG(reg, reg);
            2 OR_REG_LIT(reg, lit);
        )
    }

    fn gen_xor(&self, args: &ExprArgs) -> Vec<u8> {
        gen_patt!(
            args:
            2 XOR_REG_REG(reg, reg);
            2 XOR_REG_LIT(reg, lit);
        )
    }

    fn gen_jmp_eq(&self, args: &ExprArgs) -> Vec<u8> {
        gen_patt!(
            args:
            1 JEQ_REG(reg);
            1 JEQ_LIT(lit);
        )
    }

    fn gen_jmp_gt(&self, args: &ExprArgs) -> Vec<u8> {
        gen_patt!(
            args:
            1 JGT_REG(reg);
            1 JGT_LIT(lit);
        )
    }

    fn gen_jmp_not_eq(&self, args: &ExprArgs) -> Vec<u8> {
        gen_patt!(
            args:
            1 JNE_REG(reg);
            // XXX: fuked up
            // 1 JNE_LIT(lit);
        )
    }

    fn gen_jmp_lt(&self, args: &ExprArgs) -> Vec<u8> {
        gen_patt!(
            args:
            1 JLT_REG(reg);
            1 JLT_LIT(lit);
        )
    }

    fn gen_left_shift(&self, args: &ExprArgs) -> Vec<u8> {
        gen_patt!(
            args:
            2 LSF_REG_LIT(reg, lit);
            2 LSF_REG_REG(reg, reg);
        )
    }

    fn gen_rigth_shift(&self, args: &ExprArgs) -> Vec<u8> {
        gen_patt!(
            args:
            2 RSF_REG_LIT(reg, lit);
            2 RSF_REG_REG(reg, reg);
        )
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
