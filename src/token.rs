use std::fmt;
use crate::token_type::TokenType;

#[derive(Clone)]
pub struct Token {
    type_: TokenType,
    lexeme: String,
    literal: Option<f64>, // TODO: replace by Box once we know what traits we want
    line: usize,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: &str, literal: Option<f64>, line: usize) -> Self {
        Self {
            type_,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {:?}",
            self.type_, self.lexeme, self.literal
        )
    }
}
