use crate::token_type::TokenType;
use std::fmt;

#[derive(Clone)]
pub enum LiteralType {
    Text(String),
    Number(f64),
}

impl fmt::Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralType::Text(t) => write!(f, "{}", t),
            LiteralType::Number(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Clone)]
pub struct Token {
    type_: TokenType,
    lexeme: String,
    literal: Option<LiteralType>,
    line: usize,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: &str, literal: Option<LiteralType>, line: usize) -> Self {
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
        if let Some(literal) = &self.literal {
            write!(f, "{} {} {}", self.type_, self.lexeme, literal)
        } else {
            write!(f, "{} {} None", self.type_, self.lexeme,)
        }
    }
}
