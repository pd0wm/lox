use clap::Parser;
use std::error::Error;
use std::fmt;
use std::io::{BufRead, Write};
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename of the script to run
    #[arg()]
    script: Option<String>,
}

#[derive(Debug, Clone)]
struct LoxError {
    line: u64,
    where_s: String,
    message: String,
}

impl LoxError {
    fn new(line: u64, message: &str) -> Self {
        LoxError {
            line,
            where_s: "".to_string(),
            message: message.to_string(),
        }
    }
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[line {}] Error{}: {}",
            self.line, self.where_s, self.message
        )
    }
}

impl Error for LoxError {}

struct Lox {
    cur_line: u64,
}

impl Lox {
    fn new() -> Self {
        Self { cur_line: 0 }
    }

    fn run_file(&self, path: &std::path::Path) -> Result<(), LoxError> {
        let contents = std::fs::read_to_string(path).expect("Failed to read source");
        self.run(&contents)
    }

    fn run_prompt(&self) -> Result<(), LoxError> {
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

fn main() -> ExitCode {
    let args = Args::parse();
    let lox = Lox::new();

    let result = if let Some(script) = args.script {
        let path = std::path::Path::new(&script);
        lox.run_file(path)
    } else {
        lox.run_prompt()
    };

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            println!("{}", e);
            ExitCode::from(65)
        }
    }
}
