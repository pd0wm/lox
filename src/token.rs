use crate::token_type::TokenType;

pub struct Token {
    type_: TokenType,
    lexeme: String,
    // literal: ???,
    line: u64,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: &str, line: u64) -> Self {
        Self {
            type_,
            lexeme: lexeme.to_string(),
            line,
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        self.type_.to_string() + " " + &self.lexeme + " " + &self.line.to_string()
    }
}
