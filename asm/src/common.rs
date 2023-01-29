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

#[derive(Debug, PartialEq)]
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
