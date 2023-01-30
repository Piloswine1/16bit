use core::fmt;
use std::{iter::Peekable, str::Chars};

use crate::common::TokenEnum;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenEnum,
    pub len: u32,
}

impl<'a> fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl Token {
    pub fn new(kind: TokenEnum, len: u32) -> Self {
        Self { kind, len }
    }
}

// #[derive(Debug, PartialEq)]
// pub struct TokenWithSpan {
//     pub kind: TokenEnum,
//     pub pos: u32,
// }

// impl TokenWithSpan {
//     pub fn new(kind: TokenEnum, pos: u32) -> Self {
//         Self { kind, pos }
//     }
// }

#[derive(Debug)]
pub struct Cursor<'a> {
    pub cursor: Peekable<Chars<'a>>,
}

#[derive(Debug)]
pub enum LexerError {
    WrongHexVal,
    WrongIdent,
}

type Result<T> = std::result::Result<T, LexerError>;

pub fn is_valid_id_continue(c: &char) -> bool {
    c.is_alphabetic() || c.is_numeric()
}

pub fn tokenize(input: &str) -> impl Iterator<Item = TokenEnum> + '_ {
    let mut lexer = Cursor::new(input);
    std::iter::from_fn(move || {
        let token = lexer.parse_token();
        // println!("parsed: {:?}", token);

        if token.kind != TokenEnum::EOF {
            Some(token.kind)
        } else {
            None
        }
    })
}

pub fn tokenize_old(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut lexer = Cursor::new(input);
    std::iter::from_fn(move || {
        let token = lexer.parse_token();
        // println!("parsed: {:?}", token);

        if token.kind != TokenEnum::EOF {
            Some(token)
        } else {
            None
        }
    })
}

pub fn tokenize_expr(input: &str) -> Peekable<impl Iterator<Item = Token> + '_> {
    tokenize_old(input)
        .filter(|x| x.kind != TokenEnum::Whitespace && x.kind != TokenEnum::NewLine)
        .peekable()
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            cursor: input.chars().peekable(),
        }
    }

    // TODO: think of reg lexing
    pub fn parse_token(&mut self) -> Token {
        let first_char = self.cursor.next();
        // println!("to parse: {:?}", first_char);

        match first_char {
            Some('&') => self.parse_mem(),
            Some('$') => self.parse_lit(),
            Some(',') => Token::new(TokenEnum::Comma, 1),
            Some('(') => Token::new(TokenEnum::OpenParen, 1),
            Some(')') => Token::new(TokenEnum::CloseParen, 1),
            Some('[') => Token::new(TokenEnum::OpenBracket, 1),
            Some(']') => Token::new(TokenEnum::CloseBracket, 1),
            Some('!') => Token::new(TokenEnum::Neg, 1),
            Some(':') => Token::new(TokenEnum::Colon, 1),
            Some('?') => Token::new(TokenEnum::Question, 1),
            Some(';') => Token::new(TokenEnum::Semicolon, 1),
            Some('+') => Token::new(TokenEnum::Plus, 1),
            Some('*') => Token::new(TokenEnum::Star, 1),
            Some('-') => Token::new(TokenEnum::Minus, 1),
            Some('\n') => self.parse_newline(),
            // Some('r') => match self.cursor.next() {
            //     Some('1') => Token::Reg(crate::common::Regs::R1),
            //     Some('2') => Token::Reg(crate::common::Regs::R2),
            //     Some('3') => Token::Reg(crate::common::Regs::R3),
            //     Some('4') => Token::Reg(crate::common::Regs::R4),
            //     Some('5') => Token::Reg(crate::common::Regs::R5),
            //     Some('6') => Token::Reg(crate::common::Regs::R6),
            //     Some('7') => Token::Reg(crate::common::Regs::R7),
            //     Some('8') => Token::Reg(crate::common::Regs::R8),
            //     None => Token::Ident("r".into()),
            //     // XXX: can be r0 unvalid register, mb can use as variable
            //     _ => unimplemented!(),
            // },
            Some(x) if x.is_whitespace() => self.eat_whitespace(),
            Some(c) if c.is_alphabetic() => self.parse_ident(c),
            None => Token::new(TokenEnum::EOF, 0),
            // XXX: dunno what can happen
            _ => unimplemented!(),
        }
    }

    pub fn parse_newline(&mut self) -> Token {
        let mut len = 1;
        if self.cursor.peek() == Some(&'\t') {
            self.cursor.next();
            len += 1;
        }
        Token::new(TokenEnum::NewLine, len)
    }

    pub fn parse_hex(&mut self) -> Result<(u16, u32)> {
        let mut hex = String::new();
        while let Some(x) = self.cursor.peek() {
            if x.is_digit(16) || *x == 'x' {
                hex.push(self.cursor.next().unwrap());
            } else {
                break;
            }
        }
        if hex.starts_with("0x") {
            let without_prefix = hex.trim_start_matches("0x");
            Ok((
                u16::from_str_radix(without_prefix, 16).unwrap(),
                hex.len() as u32,
            ))
        } else {
            Ok((hex.parse::<u16>().unwrap(), hex.len() as u32))
        }
    }

    fn parse_mem(&mut self) -> Token {
        match self.parse_hex() {
            Ok((x, size)) => Token::new(TokenEnum::Mem(x), size + 1),
            Err(_) => Token::new(TokenEnum::InvalidIdent, 0),
        }
    }

    fn parse_lit(&mut self) -> Token {
        match self.parse_hex() {
            Ok((x, size)) => Token::new(TokenEnum::Lit(x), size + 1),
            Err(_) => Token::new(TokenEnum::InvalidIdent, 0),
        }
    }

    fn eat_whitespace(&mut self) -> Token {
        let mut len = 1;
        while let Some(c) = self.cursor.peek() {
            if c.is_whitespace() {
                self.cursor.next();
                len += 1;
            } else {
                break;
            }
        }
        Token::new(TokenEnum::Whitespace, len)
    }

    fn parse_ident(&mut self, first_char: char) -> Token {
        let mut ident = String::from(first_char);
        while let Some(c) = self.cursor.peek() {
            // println!("peek: {:?}", c);
            if !is_valid_id_continue(c) {
                // TODO: handle invalid idents
                break;
            }
            ident.push(self.cursor.next().unwrap());
        }
        let len = ident.len() as u32;
        Token::new(TokenEnum::Ident(ident.clone()), len)
    }
}

#[test]
fn parse_mov() {
    let tokens: Vec<_> = tokenize_old("mov $10, r2").collect();
    assert_eq!(
        tokens,
        vec![
            Token::new(TokenEnum::Ident("mov".into()), 3),
            Token::new(TokenEnum::Whitespace, 1),
            Token::new(TokenEnum::Lit(10), 3),
            Token::new(TokenEnum::Comma, 1),
            Token::new(TokenEnum::Whitespace, 1),
            Token::new(TokenEnum::Ident("r2".into()), 2),
        ]
    );
}

#[test]
fn parse_multiline() {
    let tokens: Vec<_> = tokenize_old("mov $10, r2\nmov &10, acc").collect();
    assert_eq!(
        tokens,
        vec![
            Token::new(TokenEnum::Ident("mov".into()), 3),
            Token::new(TokenEnum::Whitespace, 1),
            Token::new(TokenEnum::Lit(10), 3),
            Token::new(TokenEnum::Comma, 1),
            Token::new(TokenEnum::Whitespace, 1),
            Token::new(TokenEnum::Ident("r2".into()), 2),
            Token::new(TokenEnum::NewLine, 1),
            Token::new(TokenEnum::Ident("mov".into()), 3),
            Token::new(TokenEnum::Whitespace, 1),
            Token::new(TokenEnum::Mem(10), 3),
            Token::new(TokenEnum::Comma, 1),
            Token::new(TokenEnum::Whitespace, 1),
            Token::new(TokenEnum::Ident("acc".into()), 3),
        ]
    );
}

#[test]
fn parse_expression() {
    let tokens: Vec<_> = tokenize_old("mov [&1 + !var - $10], acc").collect();
    assert_eq!(
        tokens,
        vec![
            Token::new(TokenEnum::Ident("mov".into()), 3),
            Token::new(TokenEnum::Whitespace, 1),
            Token::new(TokenEnum::OpenBracket, 1),
            Token::new(TokenEnum::Mem(1), 2),
            Token::new(TokenEnum::Whitespace, 1),
            Token::new(TokenEnum::Plus, 1),
            Token::new(TokenEnum::Whitespace, 1),
            Token::new(TokenEnum::Neg, 1),
            Token::new(TokenEnum::Ident("var".into()), 3),
            Token::new(TokenEnum::Whitespace, 1),
            Token::new(TokenEnum::Minus, 1),
            Token::new(TokenEnum::Whitespace, 1),
            Token::new(TokenEnum::Lit(10), 3),
            Token::new(TokenEnum::CloseBracket, 1),
            Token::new(TokenEnum::Comma, 1),
            Token::new(TokenEnum::Whitespace, 1),
            Token::new(TokenEnum::Ident("acc".into()), 3),
        ]
    )
}
