use crate::common::AsmAst;

#[derive(Debug)]
pub struct Parser {}

#[derive(Debug)]
pub enum ParserError {}

type ParseRes<T> = Result<T, ParserError>;
type ParseResAst = ParseRes<Vec<AsmAst>>;

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&self, _input: &str) -> ParseResAst {
        todo!()
    }
}

