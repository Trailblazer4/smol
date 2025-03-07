//! the main compiler binary. takes a source file, an optional output format (a
//! compiled executable by default), and optimization flags.
//!
//! run with `--help` for more info.

use smol::{back::*, front::*, middle::*};

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// the input file
    file: String,
    /// the output format
    #[arg(value_enum, short, long, default_value_t = Output::Asm)]
    out: Output,
    /// turn on optimizations
    #[arg(short = 'O', default_value_t = false)]
    optimize: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, ValueEnum)]
enum Output {
    /// the list of tokens
    Tokens,
    /// the ast data structure
    Ast,
    /// tiny IR in JSON format, after optimizations
    Tir,
    /// the resulting assembly code
    Asm,
}

fn get_ir(input: &str, opt: bool) -> tir::Program {
    let ast = parse(input).unwrap();
    let ir = lower(ast);
    if opt {
        optimize(ir)
    } else {
        ir
    }
}

fn main() {
    use Output::*;
    let args = Args::parse();

    let input = String::from_utf8(std::fs::read(&args.file).expect("file should be readable"))
        .expect("input characters should be utf8");

    match args.out {
        Tokens => {
            let mut lexer = lex::Lexer::new(&input);
            while let Some(token) = lexer.next() {
                println!("{token}");
            }
        }
        Ast => {
            println!("{:?}", parse(&input).unwrap());
        }
        Tir => {
            println!("{}", get_ir(&input, args.optimize))
        }
        Asm => {
            println!("{}", code_gen(get_ir(&input, args.optimize)).asm_code())
        }
    }
}
