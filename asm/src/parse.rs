use std::iter::Peekable;

use crate::{
    ast::{Expr, ExprArgs, ExprKind, S},
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
    UnknownExpr(String, u32),
}

type ParseRes<T> = Result<T, ParserError>;

impl<'a> Parser {
    pub fn new() -> Self {
        Self { line: 0 }
    }

    pub fn parse(&mut self, input: &'a str) -> ParseRes<Vec<Expr>> {
        let mut lexer = tokenize_old(input)
            .filter(|x| x.kind != TokenEnum::Whitespace)
            .peekable();

        self.line = 0;
        let mut exprs: Vec<Expr> = Vec::new();

        while let Some(tkn) = lexer.next() {
            let expr = match &tkn.kind {
                TokenEnum::Ident(_) => self.parse_ident(&mut lexer, &tkn)?,
                TokenEnum::NewLine => {
                    self.line += 1;
                    continue;
                }
                _ => return Err(self.collect_error_on_line(&mut lexer, &tkn)),
            };
            exprs.push(expr);
        }
        Ok(exprs)
    }

    pub fn collect_error_on_line(
        &self,
        lexer: &mut Peekable<impl Iterator<Item = Token>>,
        tkn: &Token,
    ) -> ParserError {
        let line: Vec<_> = lexer
            .by_ref()
            .take_while(|x| x.kind != TokenEnum::NewLine)
            .map(|x| x.kind.to_string())
            .collect();

        ParserError::UnknownExpr(
            format!(
                "Expr: {:?} on line {:?} currently unimplemented",
                tkn,
                line.join(" ")
            ),
            self.line,
        )
    }

    pub fn try_label(
        &self,
        lexer: &mut Peekable<impl Iterator<Item = Token>>,
        ident: &Token,
    ) -> Option<Expr> {
        match lexer.peek() {
            Some(Token {
                kind: TokenEnum::Comma,
                ..
            }) => {
                lexer.next();
                Some(Expr::new(ExprKind::Label, ExprArgs::Single(ident.clone())))
            }
            None => {
                let msg = format!(
                    "{:?}",
                    ParserError::UnknownExpr("False expression".into(), self.line)
                );
                panic!("{}", msg);
            }
            _ => None,
        }
    }

    pub fn parse_ident(
        &self,
        lexer: &mut Peekable<impl Iterator<Item = Token>>,
        ident: &Token,
    ) -> ParseRes<Expr> {
        let ident_into = match &ident.kind {
            TokenEnum::Ident(ident) => ident.as_str(),
            _ => unreachable!(),
        };

        if let Some(expr) = self.try_label(lexer, ident) {
            return Ok(expr);
        }

        // Pass label parsing instructions
        match ident_into {
            "mov" => self.parse_mov(lexer),
            "add" => self.parse_double_args(lexer, ExprKind::Add),
            "sub" => self.parse_double_args(lexer, ExprKind::Sub),
            "mul" => self.parse_double_args(lexer, ExprKind::Mul),
            "and" => self.parse_double_args(lexer, ExprKind::And),
            "or" => self.parse_double_args(lexer, ExprKind::Or),
            "xor" => self.parse_double_args(lexer, ExprKind::Xor),
            "call" => self.parse_single_args(lexer, ExprKind::Call),
            "push" => self.parse_single_args(lexer, ExprKind::Push),
            "pop" => self.parse_single_args(lexer, ExprKind::Pop),
            "not" => self.parse_single_args(lexer, ExprKind::Not),
            "inc" => self.parse_single_args(lexer, ExprKind::Inc),
            "dec" => self.parse_single_args(lexer, ExprKind::Dec),
            "jne" => self.parse_double_args(lexer, ExprKind::JmpNotEQ),
            "jeq" => self.parse_double_args(lexer, ExprKind::JmpEQ),
            "jlt" => self.parse_double_args(lexer, ExprKind::JmpLT),
            "jgt" => self.parse_double_args(lexer, ExprKind::JmpGT),
            "ret" => Ok(Expr::new(ExprKind::Ret, ExprArgs::NoArgs)),
            "hlt" => Ok(Expr::new(ExprKind::HLT, ExprArgs::NoArgs)),
            _ => unimplemented!(),
        }
    }

    pub fn parse_mov(&self, lexer: &mut Peekable<impl Iterator<Item = Token>>) -> ParseRes<Expr> {
        let mov_error =
            ParserError::UnknownExpr("Mov usage: mov <expr>, <reg/mem>".into(), self.line);
        let mov_error_long = ParserError::UnknownExpr(
            "Mov lit offset reg usage: mov <lit>, <reg/mem>, <reg/mem>".into(),
            self.line,
        );

        let res = match lexer.peek() {
            Some(Token {
                kind: TokenEnum::OpenBracket,
                ..
            }) => {
                lexer.next();
                let mut expr = lexer
                    .take_while(|x| x.kind != TokenEnum::CloseBracket)
                    .into_iter()
                    .peekable();

                let lhs = Self::expr_bp(&mut expr, 0)?;
                assert_eq!(lexer.next().unwrap().kind, TokenEnum::Comma);
                let rhs = lexer.next().ok_or(mov_error)?;

                Expr::new(ExprKind::Mov, ExprArgs::Complex(lhs, rhs))
            }
            None => return Err(mov_error),
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
                    Expr::new(ExprKind::Mov, ExprArgs::Triple(lhs, maybe_mhs, rhs))
                } else {
                    Expr::new(ExprKind::Mov, ExprArgs::Double(lhs, maybe_mhs))
                }
            }
        };
        Ok(res)
    }

    pub fn parse_double_args(
        &self,
        lexer: &mut Peekable<impl Iterator<Item = Token>>,
        kind: ExprKind,
    ) -> ParseRes<Expr> {
        let lhs = lexer.next().unwrap();
        assert_eq!(lexer.next().unwrap().kind, TokenEnum::Comma);
        let rhs = lexer.next().unwrap();

        Ok(Expr::new(kind, ExprArgs::Double(lhs, rhs)))
    }

    pub fn parse_single_args(
        &self,
        lexer: &mut Peekable<impl Iterator<Item = Token>>,
        kind: ExprKind,
    ) -> ParseRes<Expr> {
        let lhs = lexer.next().unwrap();

        Ok(Expr::new(kind, ExprArgs::Single(lhs)))
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
                _ => return Err(ParserError::UnknownExpr(format!("{:?}", token), 0)),
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
        vec![Expr::new(
            ExprKind::Mov,
            ExprArgs::Double(
                Token::new(TokenEnum::Lit(1), 2),
                Token::new(TokenEnum::Lit(2), 2)
            )
        )]
    );

    let parsed = parser
        .parse(
            "mov $1, r1
            add $0x000f, r1
            not r1",
        )
        .unwrap();
    assert_eq!(
        parsed,
        vec![
            Expr::new(
                ExprKind::Mov,
                ExprArgs::Double(
                    Token::new(TokenEnum::Lit(1), 2),
                    Token::new(TokenEnum::Ident("r1".into()), 2)
                )
            ),
            Expr::new(
                ExprKind::Add,
                ExprArgs::Double(
                    Token::new(TokenEnum::Lit(0x000fu16), 7),
                    Token::new(TokenEnum::Ident("r1".into()), 2)
                )
            ),
            Expr::new(
                ExprKind::Not,
                ExprArgs::Single(Token::new(TokenEnum::Ident("r1".into()), 2))
            ),
        ]
    );

    let parsed = parser.parse("mov $1, $1, r1").unwrap();
    assert_eq!(
        parsed,
        vec![Expr::new(
            ExprKind::Mov,
            ExprArgs::Triple(
                Token::new(TokenEnum::Lit(1), 2),
                Token::new(TokenEnum::Lit(1), 2),
                Token::new(TokenEnum::Ident("r1".into()), 2)
            )
        )]
    );

    let parsed = parser.parse("mov [$1 + &2], r1").unwrap();
    assert_eq!(
        parsed,
        vec![Expr::new(
            ExprKind::Mov,
            ExprArgs::Complex(
                S::Cons(
                    Token::new(TokenEnum::Plus, 1),
                    vec![
                        S::Atom(Token::new(TokenEnum::Lit(1), 2)),
                        S::Atom(Token::new(TokenEnum::Mem(2), 2)),
                    ]
                ),
                Token::new(TokenEnum::Ident("r1".into()), 2)
            )
        )]
    );
}
