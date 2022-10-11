use std::io::{BufRead, Write};

use crate::interpreter::interpret;
use crate::lox_error::LoxError;
use crate::parser::Parser;
use crate::scanner::Scanner;

#[derive(Default)]
pub struct Lox {}

impl Lox {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn run_file(&self, path: &std::path::Path) -> Result<(), LoxError> {
        let contents = std::fs::read_to_string(path).expect("Failed to read source");
        self.run(&contents)
    }

    pub fn run_prompt(&self) -> Result<(), LoxError> {
        println!("interactive mode");
        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout();

        print!("> ");
        stdout.flush().unwrap();

        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                if let Err(e) = self.run(&line) {
                    println!("{}", e);
                };
            } else {
                break;
            }
            print!("> ");
            stdout.flush().unwrap();
        }
        Ok(())
    }

    fn run(&self, source: &str) -> Result<(), LoxError> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;
        let mut parser = Parser::new(&tokens);

        let expression = parser.parse()?;
        interpret(expression)?;

        Ok(())
    }
}
