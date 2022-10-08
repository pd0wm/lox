use crate::lox_error::LoxError;
use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Default)]
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            line: 1,
            ..Default::default()
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LoxError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(TokenType::Eof, "", None, self.line));
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), LoxError>{
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                let token_type = if self.match_next('=') { TokenType::BangEqual } else { TokenType::Bang };
                self.add_token(token_type, None)
            },
            '=' => {
                let token_type = if self.match_next('=') { TokenType::EqualEqual } else { TokenType::Equal };
                self.add_token(token_type, None)
            },
            '<' => {
                let token_type = if self.match_next('=') { TokenType::LessEqual } else { TokenType::Less };
                self.add_token(token_type, None)
            },
            '>' => {
                let token_type = if self.match_next('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                self.add_token(token_type, None)
            },
            _ => Err(LoxError::new(self.line, "Unexpected character."))
        }
    }

    fn advance(&mut self) -> char{
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn add_token(&mut self, type_: TokenType, literal: Option<f64>) -> Result<(), LoxError>{
        let text = String::from_iter(&self.source[self.start..self.current]);
        self.tokens.push(Token::new(type_, &text, literal, self.line));
        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        return true;
    }
}
