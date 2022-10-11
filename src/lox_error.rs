use std::error::Error;
use std::fmt;

use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct RuntimeError {
    token: Token,
    message: String,
}

#[derive(Debug, Clone)]
pub struct ParserError {
    token: Token,
    message: String,
}

#[derive(Debug, Clone)]
pub struct ScannerError {
    line: usize,
    message: String,
}

#[derive(Debug, Clone)]
pub enum LoxError {
    Parser(ParserError),
    Scanner(ScannerError),
    Runtime(RuntimeError),
}

impl ScannerError {
    pub fn new(line: usize, message: &str) -> Self {
        Self {
            line,
            message: message.to_string(),
        }
    }

}

impl ParserError {
    pub fn new(token: Token, message: &str) -> Self {
        Self {
            token,
            message: message.to_string(),
        }
    }
}

impl RuntimeError {
    pub fn new(token: Token, message: &str) -> Self {
        Self {
            token,
            message: message.to_string(),
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n[line {}]",
            self.message, self.token.line,
        )
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.token.type_ == TokenType::Eof {
            write!(
                f,
                "[line {}] Error at end: {}",
                self.token.line, self.message
            )
        } else {
            write!(
                f,
                "[line {}] Error at '{}': {}",
                self.token.line, self.token.lexeme, self.message
            )

        }
    }
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[line {}] Error: {}",
            self.line, self.message
        )
    }
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoxError::Runtime(e) => e.fmt(f),
            LoxError::Scanner(e) => e.fmt(f),
            LoxError::Parser(e) => e.fmt(f),
        }
    }
}

impl Error for ParserError {}
impl Error for RuntimeError {}
impl Error for LoxError {}

impl From<RuntimeError> for LoxError {
    fn from(err: RuntimeError) -> LoxError {
        LoxError::Runtime(err)
    }
}

impl From<ParserError> for LoxError {
    fn from(err: ParserError) -> LoxError {
        LoxError::Parser(err)
    }
}
