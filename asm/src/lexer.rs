use std::{iter::Peekable, str::Chars};

use crate::common::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
    _input: String,
    cursor: Peekable<Chars<'a>>,
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

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            _input: input.to_string(),
            cursor: input.chars().peekable(),
        }
    }

    pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
        let mut lines = input.lines();
        let mut lexer = Lexer::new(lines.next().unwrap());
        std::iter::from_fn(move || {
            let token = lexer.parse_token();
            // println!("parsed: {:?}", token);

            if token != Token::EOF {
                Some(token)
            } else if let Some(line) = lines.next() {
                lexer = Lexer::new(line);
                Some(Token::EOL)
            } else {
                None
            }
        })
    }

    // TODO: think of reg lexing
    pub fn parse_token(&mut self) -> Token {
        let first_char = self.cursor.next();
        println!("to parse: {:?}", first_char);

        match first_char {
            Some(x) if x.is_whitespace() => self.eat_whitespace(),
            Some('&') => self.parse_mem(),
            Some('$') => self.parse_lit(),
            Some(',') => Token::Comma,
            Some('(') => Token::OpenParen,
            Some(')') => Token::CloseParen,
            Some('[') => Token::OpenBracket,
            Some(']') => Token::CloseBracket,
            Some('!') => Token::Neg,
            Some(':') => Token::Colon,
            Some('+') => Token::Plus,
            Some('*') => Token::Star,
            Some('-') => Token::Minus,
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
            Some(c) if c.is_alphabetic() => self.parse_ident(c),
            None => Token::EOF,
            // XXX: dunno what can happen
            _ => unimplemented!(),
        }
    }

    /// Implicitly we ate 'r'
    // pub fn try_parse_reg(&mut self, first_char: char) -> Token {
    //     match self.cursor.peek() {
    //         Some('_') => {
    //             self.cursor.next();
    //
    //         },
    //         None => Token::Ident("r".into()),
    //         Some(_) => self.parse_ident(first_char),
    //     }
    // }

    pub fn parse_hex(&mut self) -> Result<u16> {
        let mut hex = String::new();
        while let Some(x) = self.cursor.peek() {
            if x.is_digit(16) {
                hex.push(self.cursor.next().unwrap());
            } else {
                break;
            }
        }
        Ok(hex.parse::<u16>().unwrap())
    }

    fn parse_mem(&mut self) -> Token {
        match self.parse_hex() {
            Ok(x) => Token::Mem(x),
            Err(_) => Token::InvalidIdent,
        }
    }

    fn parse_lit(&mut self) -> Token {
        match self.parse_hex() {
            Ok(x) => Token::Lit(x),
            Err(_) => Token::InvalidIdent,
        }
    }

    fn eat_whitespace(&mut self) -> Token {
        while let Some(c) = self.cursor.peek() {
            if c.is_whitespace() {
                self.cursor.next();
            } else {
                break;
            }
        }
        Token::Whitespace
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
        Token::Ident(ident)
    }
}

#[test]
fn parse_mov() {
    let tokens: Vec<_> = Lexer::tokenize("mov $10, r2").collect();
    assert_eq!(
        tokens,
        vec![
            Token::Ident("mov".into()),
            Token::Whitespace,
            Token::Lit(10),
            Token::Comma,
            Token::Whitespace,
            Token::Ident("r2".into()),
        ]
    );
}

#[test]
fn parse_multiline() {
    let tokens: Vec<_> = Lexer::tokenize("mov $10, r2\nmov &10, acc").collect();
    assert_eq!(
        tokens,
        vec![
            Token::Ident("mov".into()),
            Token::Whitespace,
            Token::Lit(10),
            Token::Comma,
            Token::Whitespace,
            Token::Ident("r2".into()),
            Token::EOL,
            Token::Ident("mov".into()),
            Token::Whitespace,
            Token::Mem(10),
            Token::Comma,
            Token::Whitespace,
            Token::Ident("acc".into()),
        ]
    );
}
