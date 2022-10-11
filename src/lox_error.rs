use std::error::Error;
use std::fmt;

use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct LoxError {
    line: usize,
    where_: String,
    message: String,
}

impl LoxError {
    pub fn new(line: usize, message: &str) -> Self {
        LoxError {
            line,
            where_: "".to_string(),
            message: message.to_string(),
        }
    }

    pub fn error(token: Token, message: &str) -> Self {
        if token.type_ == TokenType::Eof {
            LoxError {
                line: token.line,
                where_: " at end".to_string(),
                message: message.to_string(),
            }
        } else {
            LoxError {
                line: token.line,
                where_: format!(" at '{}'", token.lexeme),
                message: message.to_string(),
            }
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
