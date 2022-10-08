use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct LoxError {
    line: u64,
    where_: String,
    message: String,
}

impl LoxError {
    pub fn new(line: u64, message: &str) -> Self {
        LoxError {
            line,
            where_: "".to_string(),
            message: message.to_string(),
        }
    }
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[line {}] Error{}: {}",
            self.line, self.where_, self.message
        )
    }
}

impl Error for LoxError {}
