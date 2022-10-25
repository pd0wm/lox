use std::io::{BufRead, Write};

use crate::interpreter::Interpreter;
use crate::lox_error::LoxError;
use crate::parser::Parser;
use crate::scanner::Scanner;

pub struct Lox {
    interpreter: Interpreter,
}

impl Lox {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
        }
    }

    pub fn run_file(&mut self, path: &std::path::Path) -> Result<(), LoxError> {
        let contents = std::fs::read_to_string(path).expect("Failed to read source");
        self.run(&contents)
    }

    pub fn run_prompt(&mut self) -> Result<(), LoxError> {
        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout();

        print!("> ");
        stdout.flush().unwrap();

        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                if let Err(e) = self.run(&line) {
                    eprintln!("{}", e);
                };
            } else {
                break;
            }
            print!("> ");
            stdout.flush().unwrap();
        }
        Ok(())
    }

    fn run(&mut self, source: &str) -> Result<(), LoxError> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;
        let mut parser = Parser::new(&tokens);

        let statements = parser.parse()?;
        self.interpreter.interpret(statements)?;

        Ok(())
    }
}
