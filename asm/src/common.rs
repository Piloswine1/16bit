use core::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Regs {
    ACC = 1,
    R1 = 2,
    R2 = 3,
    R3 = 4,
    R4 = 5,
    R5 = 6,
    R6 = 7,
    R7 = 8,
    R8 = 9,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenEnum {
    Whitespace,
    Comma,
    Colon,
    Semicolon,
    // logic
    Or,
    And,
    Neg,
    // arithm
    Plus,
    Star,
    Minus,
    // vars
    Lit(u16),
    Mem(u16),
    Reg(Regs),
    // params
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Question,
    // ident
    Ident(String),
    // invalid
    InvalidIdent,
    EOF,
    NewLine,
}

impl fmt::Display for TokenEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenEnum::Ident(ident) => write!(f, "{}", ident),
            TokenEnum::Lit(lit) => write!(f, "{}", lit),
            TokenEnum::Mem(mem) => write!(f, "&{}", mem),
            TokenEnum::Reg(reg) => write!(f, "{:?}", reg),
            TokenEnum::Comma => write!(f, ","),
            TokenEnum::Plus => write!(f, "+"),
            TokenEnum::Minus => write!(f, "-"),
            TokenEnum::Star => write!(f, "*"),
            TokenEnum::Question => write!(f, "?"),
            TokenEnum::Colon => write!(f, ":"),
            _ => return Err(fmt::Error),
        }
    }
}
