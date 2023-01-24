use crate::common::*;

#[derive(Debug)]
pub enum ParseErr {
    EmptyInstruction,
    UnsupportedInstruction(String),
    WrongArg(String),
}
type ParseRes = Result<Vec<ASMCommand>, ParseErr>;
type ParseSingleRes = Result<ASMCommand, ParseErr>;

pub struct Parser;

fn parse_arg(arg: &str) -> Result<ArgType, ParseErr> {
    let arg_lowercase = arg.to_lowercase();
    let mut arg_as_chars = arg_lowercase.chars();
    match arg_as_chars.next() {
        Some(x) if x == '&' => {
            let value = arg_as_chars.as_str();
            let parsed = u16::from_str_radix(value, 16).map_err(|_| ParseErr::WrongArg(arg.to_owned()))?;
            Ok(ArgType::Mem(parsed))
        }
        Some(x) if x == '$' => {
            let value = arg_as_chars.as_str();
            let parsed = u16::from_str_radix(value, 16).map_err(|_| ParseErr::WrongArg(arg.to_owned()))?;
            Ok(ArgType::Hex(parsed))
        }
        Some(x) if x == 'a' => {
            if arg_lowercase == "acc" {
               Ok(ArgType::Reg(Regs::ACC)) 
            } else {
                Err(ParseErr::WrongArg(arg.to_owned()))
            }
        }
        Some(x) if x == 'r' => {
            let value = arg_as_chars.as_str();
            let parsed = u16::from_str_radix(value, 10).map_err(|_| ParseErr::WrongArg(arg.to_owned()))?;
            match parsed {
                1 => Ok(ArgType::Reg(Regs::R1)),
                2 => Ok(ArgType::Reg(Regs::R2)),
                3 => Ok(ArgType::Reg(Regs::R3)),
                4 => Ok(ArgType::Reg(Regs::R4)),
                5 => Ok(ArgType::Reg(Regs::R5)),
                6 => Ok(ArgType::Reg(Regs::R6)),
                7 => Ok(ArgType::Reg(Regs::R7)),
                8 => Ok(ArgType::Reg(Regs::R8)),
                _ => Err(ParseErr::WrongArg(arg.to_owned()))
            }
        }
        _ => Err(ParseErr::WrongArg(arg.to_owned())),
    }
}

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    pub fn parse_mov(&self, line: &str) -> ParseSingleRes {
        // TODO: add support of parsing expressions
        // TODO: mb use parser combinator to parse expressions
        let args: Vec<_> = line.split_whitespace().collect();
        let first_arg = parse_arg(args.get(0).unwrap())?;
        let second_arg = parse_arg(args.get(1).unwrap())?;
        Ok(ASMCommand::Mov(LVal::from(first_arg), RVal::try_from(second_arg).unwrap()))
    }

    pub fn parse(&self, input: &str) -> ParseRes {
        let mut commands: Vec<ASMCommand> = Vec::new();
        for line in input.lines() {
            let prepared_line = line.trim();
            println!("Parsing: {}", prepared_line);
            let (cmd, rest_line) = prepared_line.split_once(" ").unwrap_or((prepared_line, ""));
            match cmd.to_lowercase().as_str() {
                "mov" => {
                    commands.push(self.parse_mov(rest_line)?);
                },
                "hlt" => {
                    commands.push(ASMCommand::Hlt);
                }
                "" => return Err(ParseErr::EmptyInstruction),
                _ => return Err(ParseErr::UnsupportedInstruction(line.to_owned())),
            }
        }
        Ok(commands)
    }
}
