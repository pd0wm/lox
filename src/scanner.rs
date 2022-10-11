use crate::lox_error::LoxError;
use crate::token::{Literal, Token};
use crate::token_type::TokenType;
use std::collections::HashMap;

#[derive(Default)]
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        let mut s = Self {
            source: source.chars().collect(),
            line: 1,
            ..Default::default()
        };

        // Initialize keywords HashMap
        s.keywords.insert("and".to_string(), TokenType::And);
        s.keywords.insert("class".to_string(), TokenType::Class);
        s.keywords.insert("else".to_string(), TokenType::Else);
        s.keywords.insert("false".to_string(), TokenType::False);
        s.keywords.insert("for".to_string(), TokenType::For);
        s.keywords.insert("fun".to_string(), TokenType::Fun);
        s.keywords.insert("if".to_string(), TokenType::If);
        s.keywords.insert("nil".to_string(), TokenType::Nil);
        s.keywords.insert("or".to_string(), TokenType::Or);
        s.keywords.insert("print".to_string(), TokenType::Print);
        s.keywords.insert("return".to_string(), TokenType::Return);
        s.keywords.insert("super".to_string(), TokenType::Super);
        s.keywords.insert("this".to_string(), TokenType::This);
        s.keywords.insert("true".to_string(), TokenType::True);
        s.keywords.insert("var".to_string(), TokenType::Var);
        s.keywords.insert("while".to_string(), TokenType::While);

        s
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LoxError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "", None, self.line));
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();

        match c {
            // Short Lexemes
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
                let token_type = if self.match_next('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type, None)
            }
            '=' => {
                let token_type = if self.match_next('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type, None)
            }
            '<' => {
                let token_type = if self.match_next('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type, None)
            }
            '>' => {
                let token_type = if self.match_next('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type, None)
            }

            // Longer Lexemes
            '/' => {
                if self.match_next('/') {
                    // Comment, ignore rest of line
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(())
                } else {
                    self.add_token(TokenType::Slash, None)
                }
            }

            // Whitespace
            ' ' => Ok(()),
            '\r' => Ok(()),
            '\t' => Ok(()),

            '\n' => {
                self.line += 1;
                Ok(())
            }

            // Strings
            '"' => self.string(),

            // Number?
            _ => {
                if c.is_ascii_digit() {
                    self.number()
                } else if c.is_ascii_alphabetic() {
                    self.identifier()
                } else {
                    Err(LoxError::new(self.line, "Unexpected character."))
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn add_token(&mut self, type_: TokenType, literal: Option<Literal>) -> Result<(), LoxError> {
        let text = String::from_iter(&self.source[self.start..self.current]);
        self.tokens
            .push(Token::new(type_, &text, literal, self.line));
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

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source[self.current])
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            None
        } else {
            Some(self.source[self.current + 1])
        }
    }

    fn string(&mut self) -> Result<(), LoxError> {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LoxError::new(self.line, "Unterminated string."));
        }

        // Eat the closing "
        self.advance();

        // Extract string
        let val = String::from_iter(&self.source[self.start + 1..self.current - 1]);

        self.add_token(TokenType::String, Some(Literal::String(val)))
    }

    fn number(&mut self) -> Result<(), LoxError> {
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
        }

        // Consume part after decimal separator
        if self.peek() == Some('.') && self.peek_next().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();

            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.advance();
            }
        }
        let val = String::from_iter(&self.source[self.start..self.current]);
        let val: f64 = val.parse().unwrap();

        self.add_token(TokenType::Number, Some(Literal::Number(val)))
    }

    fn identifier(&mut self) -> Result<(), LoxError> {
        while self.peek().is_some_and(|c| c.is_ascii_alphanumeric()) {
            self.advance();
        }

        let val = String::from_iter(&self.source[self.start..self.current]);
        if let Some(keyword) = self.keywords.get(&val) {
            self.add_token(keyword.clone(), None)
        } else {
            self.add_token(TokenType::Identifier, None)
        }
    }
}
