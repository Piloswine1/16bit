use std::iter::Peekable;

use crate::{
    ast::{Expr, S},
    common::TokenEnum,
    lexer::{tokenize_old, Token},
};

#[derive(Debug, Clone)]
pub struct Parser {
    line: u32,
}

#[derive(Debug, Clone)]
pub enum ParserError {
    EmptyExpr,
    UnknownExpr(String),
}

type ParseRes<T> = Result<T, ParserError>;

impl Parser {
    pub fn new() -> Self {
        Self { line: 0 }
    }

    pub fn parse<'a>(&mut self, input: &'a str) -> ParseRes<Vec<Expr>> {
        let mut lexer = tokenize_old(input)
            .filter(|x| x.kind != TokenEnum::Whitespace)
            .peekable();

        self.line = 0;
        let mut exprs: Vec<Expr> = Vec::new();

        while let Some(tkn) = lexer.next() {
            let expr = match tkn.kind {
                TokenEnum::Ident(ident) => self.parse_ident(&mut lexer, ident.as_str()),
                TokenEnum::NewLine => {
                    self.line += 1;
                    continue;
                }
                _ => {
                    let line: Vec<_> = lexer
                        .by_ref()
                        .take_while(|x| x.kind != TokenEnum::NewLine)
                        .map(|x| x.kind.to_string())
                        .collect();

                    Expr::Unimplemended(
                        format!(
                            "Expr: {:?} on line {:?} currently unimplemented",
                            tkn,
                            line.join(" ")
                        ),
                        self.line,
                    )
                }
            };
            exprs.push(expr);
        }
        Ok(exprs)
    }

    pub fn parse_ident(
        &self,
        lexer: &mut Peekable<impl Iterator<Item = Token>>,
        ident: &str,
    ) -> Expr {
        match lexer.peek() {
            Some(Token {
                kind: TokenEnum::Comma,
                ..
            }) => {
                lexer.next();
                return Expr::Label(ident.into());
            }
            None => return Expr::WrongExpr("False expression".into(), self.line),
            _ => (),
        };

        // Pass label parsing instructions
        let expr = match ident {
            "mov" => self.parse_mov(lexer),
            "add" => self.parse_add(lexer),
            "call" => self.parse_call(lexer),
            "push" => self.parse_push(lexer),
            "pop" => self.parse_pop(lexer),
            "jne" => self.parse_jne(lexer),
            "jeq" => self.parse_jeq(lexer),
            // TODO: jlt/jgt
            "ret" => Ok(Expr::Ret),
            "hlt" => Ok(Expr::HLT),
            _ => unimplemented!(),
        };

        match expr {
            Err(ParserError::UnknownExpr(msg)) => Expr::WrongExpr(msg, self.line),
            Err(err) => Expr::UnknownExpr(format!("{:?}", err), self.line),
            Ok(expr) => expr,
        }
    }

    pub fn parse_mov(&self, lexer: &mut Peekable<impl Iterator<Item = Token>>) -> ParseRes<Expr> {
        let mov_error = ParserError::UnknownExpr("Mov usage: mov <expr>, <reg/mem>".into());
        let mov_error_long = ParserError::UnknownExpr(
            "Mov lit offset reg usage: mov <lit>, <reg/mem>, <reg/mem>".into(),
        );

        let res = match lexer.peek() {
            Some(tkn) => match tkn.kind {
                TokenEnum::OpenBracket => {
                    lexer.next();
                    let mut expr = lexer
                        .take_while(|x| x.kind != TokenEnum::CloseBracket)
                        .into_iter()
                        .peekable();

                    let lhs = Self::expr_bp(&mut expr, 0)?;
                    assert_eq!(lexer.next().unwrap().kind, TokenEnum::Comma);
                    let rhs = lexer.next().ok_or(mov_error)?;

                    Expr::MovComplex(lhs, rhs)
                }
                _ => {
                    // TODO: validate
                    let lhs = lexer.next().unwrap();
                    assert_eq!(lexer.next().unwrap().kind, TokenEnum::Comma);
                    // TODO: validate is mem or reg
                    let maybe_mhs = lexer.next().ok_or(mov_error)?;
                    if let Some(Token {
                        kind: TokenEnum::Comma,
                        ..
                    }) = lexer.peek()
                    {
                        lexer.next();
                        let rhs = lexer.next().ok_or(mov_error_long)?;
                        Expr::MovLitOff(lhs, maybe_mhs, rhs)
                    } else {
                        Expr::Mov(lhs, maybe_mhs)
                    }
                }
            },
            None => return Err(mov_error),
        };
        Ok(res)
    }

    pub fn parse_add(&self, lexer: &mut Peekable<impl Iterator<Item = Token>>) -> ParseRes<Expr> {
        // let add_error = ParserError::UnknownExpr("Add usage: add <expr>, <reg/mem>".into());

        let lhs = lexer.next().unwrap();
        assert_eq!(lexer.next().unwrap().kind, TokenEnum::Comma);
        let rhs = lexer.next().unwrap();

        Ok(Expr::Add(lhs, rhs))
    }

    fn parse_call(&self, lexer: &mut Peekable<impl Iterator<Item = Token>>) -> ParseRes<Expr> {
        let lhs = lexer.next().unwrap();

        Ok(Expr::Call(lhs))
    }

    fn parse_push(
        &self,
        lexer: &mut Peekable<impl Iterator<Item = Token>>,
    ) -> Result<Expr, ParserError> {
        let lhs = lexer.next().unwrap();

        Ok(Expr::Push(lhs))
    }

    fn parse_pop(
        &self,
        lexer: &mut Peekable<impl Iterator<Item = Token>>,
    ) -> Result<Expr, ParserError> {
        let lhs = lexer.next().unwrap();

        Ok(Expr::Pop(lhs))
    }

    fn parse_jne(
        &self,
        lexer: &mut Peekable<impl Iterator<Item = Token>>,
    ) -> Result<Expr, ParserError> {
        let lhs = lexer.next().unwrap();
        assert_eq!(lexer.next().unwrap().kind, TokenEnum::Comma);
        let rhs = lexer.next().unwrap();

        Ok(Expr::JmpNotEQ(lhs, rhs))
    }

    fn parse_jeq(
        &self,
        lexer: &mut Peekable<impl Iterator<Item = Token>>,
    ) -> Result<Expr, ParserError> {
        let lhs = lexer.next().unwrap();
        assert_eq!(lexer.next().unwrap().kind, TokenEnum::Comma);
        let rhs = lexer.next().unwrap();

        Ok(Expr::JmpEQ(lhs, rhs))
    }

    #[allow(dead_code)]
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

#[test]
fn parse_expr() {
    let mut parser = Parser::new();
    let parsed = parser.parse("mov $1, $2").unwrap();
    assert_eq!(
        parsed,
        vec![Expr::Mov(
            Token {
                kind: TokenEnum::Lit(1),
                len: 2
            },
            Token {
                kind: TokenEnum::Lit(2),
                len: 2
            }
        )]
    );

    let parsed = parser.parse("mov $1, &2, $2").unwrap();
    assert_eq!(
        parsed,
        vec![Expr::MovLitOff(
            Token {
                kind: TokenEnum::Lit(1),
                len: 2
            },
            Token {
                kind: TokenEnum::Mem(2),
                len: 2
            },
            Token {
                kind: TokenEnum::Lit(2),
                len: 2
            }
        )]
    );

    let parsed = parser
        .parse(
            "mov $1, r1
            add $1, r1",
        )
        .unwrap();
    assert_eq!(
        parsed,
        vec![
            Expr::Mov(
                Token {
                    kind: TokenEnum::Lit(1),
                    len: 2
                },
                Token {
                    kind: TokenEnum::Ident("r1".into()),
                    len: 2
                }
            ),
            Expr::Add(
                Token {
                    kind: TokenEnum::Lit(1),
                    len: 2
                },
                Token {
                    kind: TokenEnum::Ident("r1".into()),
                    len: 2
                }
            )
        ]
    );

    let parsed = parser.parse("mov [$1 + &2 * $1], r2").unwrap();
    assert_eq!(
        parsed,
        vec![Expr::MovComplex(
            S::Cons(
                Token {
                    kind: TokenEnum::Plus,
                    len: 1
                },
                vec![
                    S::Atom(Token {
                        kind: TokenEnum::Lit(1),
                        len: 2
                    }),
                    S::Cons(
                        Token {
                            kind: TokenEnum::Star,
                            len: 1
                        },
                        vec![
                            S::Atom(Token {
                                kind: TokenEnum::Mem(2),
                                len: 2
                            }),
                            S::Atom(Token {
                                kind: TokenEnum::Lit(1),
                                len: 2
                            })
                        ]
                    )
                ]
            ),
            Token {
                kind: TokenEnum::Ident("r2".into()),
                len: 2
            }
        )]
    );

    let parsed = parser.parse("call $0x000f").unwrap();
    assert_eq!(
        parsed,
        vec![Expr::Call(Token {
            kind: TokenEnum::Lit(0x000fu16),
            len: 7
        })]
    );

    let parsed = parser
        .parse(
            "push $1
            pop r1",
        )
        .unwrap();
    assert_eq!(
        parsed,
        vec![
            Expr::Push(Token {
                kind: TokenEnum::Lit(1),
                len: 2
            }),
            Expr::Pop(Token {
                kind: TokenEnum::Ident("r1".into()),
                len: 2
            })
        ]
    );
}
