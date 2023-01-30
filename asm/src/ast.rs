use core::fmt;

use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Mov,
    Add,
    Sub,
    Mul,
    And,
    Or,
    Xor,
    Inc,
    Dec,
    Not,
    JmpEQ,
    JmpNotEQ,
    JmpGT,
    JmpLT,
    Push,
    Pop,
    Call,
    Ret,
    HLT,

    Label,
    // // XXX: maybe move to typechecker
    // WrongExpr(String, u32),
    // UnknownExpr(String, u32),
    // Unimplemended(String, u32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprArgs {
    NoArgs,
    Single(Token),
    Complex(S, Token),
    Double(Token, Token),
    Triple(Token, Token, Token),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub args: ExprArgs,
}

impl Expr {
    pub fn new(kind: ExprKind, args: ExprArgs) -> Self {
        Self { kind, args }
    }
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
