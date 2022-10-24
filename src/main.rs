#![feature(is_some_and)]

use clap::Parser;
use lox_error::LoxError;
use std::process::ExitCode;

mod ast;
mod environment;
mod interpreter;
mod lox;
mod lox_error;
mod parser;
mod scanner;
mod token;
mod token_type;

use crate::lox::Lox;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename of the script to run
    #[arg()]
    script: Option<String>,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let mut lox = Lox::new();

    let result = if let Some(script) = args.script {
        let path = std::path::Path::new(&script);
        lox.run_file(path)
    } else {
        lox.run_prompt()
    };

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(LoxError::Scanner(e)) => {
            eprintln!("{}", e);
            ExitCode::from(65)
        }
        Err(LoxError::Parser(e)) => {
            eprintln!("{}", e);
            ExitCode::from(65)
        }
        Err(LoxError::Runtime(e)) => {
            eprintln!("{}", e);
            ExitCode::from(70)
        }
    }
}
