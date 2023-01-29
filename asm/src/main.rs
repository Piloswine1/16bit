pub mod codegen;
pub mod common;
pub mod instructions;
pub mod parse;
pub mod lexer;
pub mod ast;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::PathBuf,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    input: PathBuf,
    /// Dump ast
    #[arg(short, long, default_value_t = false)]
    dump: bool,
    /// Output file name
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    use parse::Parser;

    let args = Args::parse();
    let mut parser = Parser::new();

    let file = File::open(args.input)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let parsed = parser.parse(contents.as_str()).unwrap();

    if args.dump {
        println!("{:?}", parsed);
        return Ok(());
    }

    // let binary = codegen.code_gen(parsed.as_slice()).unwrap();

    // let out_name = args.output.unwrap_or(PathBuf::from("a.out"));
    // let mut out_file = File::create(&out_name)?;
    // out_file.write_all(binary.as_slice())?;
    //
    // println!("Wrote {} bytes to {}", binary.len(), out_name.display());

    Ok(())
}
