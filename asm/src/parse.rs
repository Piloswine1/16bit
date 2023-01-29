use std::iter::Peekable;

use crate::{
    ast::{Expr, S},
    common::TokenEnum,
    lexer::{Cursor, Token},
};

#[derive(Debug)]
pub struct Parser<'a> {
    cursor: Cursor<'a>,
}

#[derive(Debug)]
pub enum ParserError {
    EmptyExpr,
    UnknownExpr(String),
}

type ParseRes<T> = Result<T, ParserError>;

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Self {
            cursor: Cursor::new(""),
        }
    }

    pub fn parse(&mut self, input: &'a str) -> ParseRes<Vec<Expr>> {
        self.cursor = Cursor::new(input);
        Ok(vec![])
    }

    fn expr_bp(lexer: &mut Peekable<impl Iterator<Item = Token>>, min_bp: u8) -> ParseRes<S> {
        let token = lexer.next().ok_or(ParserError::EmptyExpr)?;
        let mut lhs = match Self::prefix_binding_power(&token) {
            Some(((), r_bp)) => {
                let rhs = Self::expr_bp(lexer, r_bp)?;
                S::Cons(token, vec![rhs])
            }
            None => match token.kind {
                TokenEnum::OpenParen => {
                    let lhs = Self::expr_bp(lexer, 0)?;
                    assert_eq!(lexer.next().unwrap().kind, TokenEnum::CloseParen);
                    lhs
                }
                TokenEnum::Mem(_) | TokenEnum::Lit(_) | TokenEnum::Ident(_) => S::Atom(token),
                _ => return Err(ParserError::UnknownExpr(format!("{:?}", token))),
            },
        };

        loop {
            let op = match lexer.peek() {
                Some(tkn) => tkn,
                None => break,
            };

            if let Some((l_bp, r_bp)) = Self::infix_binding_power(op) {
                if l_bp < min_bp {
                    break;
                }

                let op = lexer.next().unwrap();

                lhs = if op.kind == TokenEnum::Question {
                    let mhs = Self::expr_bp(lexer, 0)?;
                    assert_eq!(lexer.next().unwrap().kind, TokenEnum::Colon);
                    let rhs = Self::expr_bp(lexer, r_bp)?;
                    S::Cons(op, vec![lhs, mhs, rhs])
                } else {
                    let rhs = Self::expr_bp(lexer, r_bp)?;
                    S::Cons(op, vec![lhs, rhs])
                };

                continue;
            }

            break;
        }

        Ok(lhs)
    }

    fn prefix_binding_power(op: &Token) -> Option<((), u8)> {
        let res = match op.kind {
            TokenEnum::Plus | TokenEnum::Minus => ((), 9),
            _ => return None,
        };
        Some(res)
    }

    fn infix_binding_power(op: &Token) -> Option<(u8, u8)> {
        let res = match op.kind {
            TokenEnum::Question => (2, 1),
            TokenEnum::Plus | TokenEnum::Minus => (3, 4),
            TokenEnum::Star => (5, 6),
            // TODO: can add '.'
            _ => return None,
        };
        Some(res)
    }

    // TODO: may be postfix
}

#[test]
fn tests() {
    use crate::lexer::tokenize_expr;

    let mut tokens = tokenize_expr("&1");
    let s = Parser::expr_bp(&mut tokens, 0).unwrap();
    assert_eq!(s.to_string(), "&1");

    let mut tokens = tokenize_expr("&1 + &2 * &3");
    let s = Parser::expr_bp(&mut tokens, 0).unwrap();
    assert_eq!(s.to_string(), "(+ &1 (* &2 &3))");

    let mut tokens = tokenize_expr("a + b * c * d + e");
    let s = Parser::expr_bp(&mut tokens, 0).unwrap();
    assert_eq!(s.to_string(), "(+ (+ a (* (* b c) d)) e)");

    let mut tokens = tokenize_expr("--$1 * $2");
    let s = Parser::expr_bp(&mut tokens, 0).unwrap();
    assert_eq!(s.to_string(), "(* (- (- 1)) 2)");

    let mut tokens = tokenize_expr("((((($0)))))");
    let s = Parser::expr_bp(&mut tokens, 0).unwrap();
    assert_eq!(s.to_string(), "0");

    let mut tokens = tokenize_expr(
        "a ? b :
        c ? d :
        e",
    );
    let s = Parser::expr_bp(&mut tokens, 0).unwrap();
    assert_eq!(s.to_string(), "(? a b (? c d e))");

    let mut tokens = tokenize_expr("(&1 + &2) * &3");
    let s = Parser::expr_bp(&mut tokens, 0).unwrap();
    assert_eq!(s.to_string(), "(* (+ &1 &2) &3)");
}
