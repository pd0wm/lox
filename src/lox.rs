use std::io::{BufRead, Write};

use crate::lox_error::LoxError;

pub struct Lox {
    cur_line: u64,
}

impl Lox {
    pub fn new() -> Self {
        Self { cur_line: 0 }
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
                self.run(&line)?;
            } else {
                break;
            }
            print!("> ");
            stdout.flush().unwrap();
        }
        Ok(())
    }

    fn run(&self, source: &str) -> Result<(), LoxError> {
        println!("Running {}", source);
        Err(LoxError::new(self.cur_line, "Help!"))
    }
}
