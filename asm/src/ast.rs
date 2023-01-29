use core::fmt;

use crate::{common::Regs, lexer::Token};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Mov(Token, Token),
    Add(Token, Token),
    Sub(Token, Token),
    Mul(Token, Token),
    Inc(Token),
    Dec(Token),
    Not(Token),
    JmpEQ(Token, Token),
    JmpNotEQ(Token, Token),
    JmpGT(Token, Token),
    JmpLE(Token, Token),
    Push(Token),
    Pop(Token),
    Call(Token),
    Ret,
    HLT,

    MovComplex(S, Token),
    MovLitOff(Token, Token, Token),

    Label(String),
    // XXX: maybe move to typechecker
    WrongExpr(String, u32),
    UnknownExpr(String, u32),
    Unimplemended(String, u32),
}

// #[derive(Debug)]
// enum MathExprKind {
//     Plus,
//     Minus,
//     Neg,
//     Star,
// }
//
// #[derive(Debug)]
// struct MathExpr {
//     kind: MathExprKind,
//     args: (),
// }

#[derive(Debug, Clone, PartialEq)]
pub enum S {
    Atom(Token),
    Cons(Token, Vec<S>),
}

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Atom(i) => write!(f, "{}", i),
            S::Cons(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}
