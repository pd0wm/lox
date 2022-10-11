use crate::token_type::TokenType;
use std::fmt;

#[derive(Clone)]
pub enum Literal {
    None,
    Bool(bool),
    String(String),
    Number(f64),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::String(t) => write!(f, "{}", t),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::None => write!(f, "nil"),
        }
    }
}

#[derive(Clone)]
pub struct Token {
    pub type_: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: &str, literal: Option<Literal>, line: usize) -> Self {
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
