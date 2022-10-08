use clap::Parser;
use std::process::ExitCode;

mod lox;
mod lox_error;

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
