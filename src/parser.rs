use crate::ast::{Expr, Stmt};
use crate::lox_error::{LoxError, ParserError};
use crate::token::{Literal, Token};
use crate::token_type::TokenType;

#[derive(Default, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: &Vec<Token>) -> Self {
        Self {
            tokens: tokens.clone(),
            ..Default::default()
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if let Some(statement) = self.declaration() {
                statements.push(statement);
            }
            // statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Option<Stmt> {
        let result = {
            if self.match_(&[TokenType::Var]) {
                self.var_declaration()
            } else {
                self.statement()
            }
        };

        match result {
            Ok(statement) => Some(statement),
            Err(e) => {
                println!("{}", e);
                self.synchronize();
                None
            }
        }
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.match_(&[TokenType::Print]) {
            return self.print_statement();
        }

        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(Stmt::Print { expression: value })
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let initializer = if self.match_(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::Var { name, initializer })
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;

        Ok(Stmt::Expression { expression: expr })
    }

    fn expression(&mut self) -> Result<Box<Expr>, LoxError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Box<Expr>, LoxError> {
        let expr = self.equality()?;

        if self.match_(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable { name } = *expr {
                return Ok(Box::new(Expr::Assign { name, value }));
            }

            return Err(ParserError::new(equals, "Invalid assignement target.").into());
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Box<Expr>, LoxError> {
        let mut expr = self.comparison()?;
        while self.match_(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<Expr>, LoxError> {
        let mut expr = self.term()?;

        while self.match_(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expr>, LoxError> {
        let mut expr = self.factor()?;

        while self.match_(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<Expr>, LoxError> {
        if self.match_(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Box::new(Expr::Unary { operator, right }))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Box<Expr>, LoxError> {
        if self.match_(&[TokenType::False]) {
            Ok(Box::new(Expr::Literal {
                value: Literal::Bool(false),
            }))
        } else if self.match_(&[TokenType::True]) {
            Ok(Box::new(Expr::Literal {
                value: Literal::Bool(true),
            }))
        } else if self.match_(&[TokenType::Nil]) {
            Ok(Box::new(Expr::Literal {
                value: Literal::None,
            }))
        } else if self.match_(&[TokenType::Number, TokenType::String]) {
            Ok(Box::new(Expr::Literal {
                value: self.previous().literal.unwrap(),
            }))
        } else if self.match_(&[TokenType::Identifier]) {
            Ok(Box::new(Expr::Variable {
                name: self.previous(),
            }))
        } else if self.match_(&[TokenType::LeftParen]) {
            let expression = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            Ok(Box::new(Expr::Grouping { expression }))
        } else {
            Err(ParserError::new(self.peek(), "Expect expression.").into())
        }
    }

    fn factor(&mut self) -> Result<Box<Expr>, LoxError> {
        let mut expr = self.unary()?;

        while self.match_(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Box::new(Expr::Binary {
                left: expr,
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn match_(&mut self, types: &[TokenType]) -> bool {
        for type_ in types {
            if self.check(type_.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, type_: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().type_ == type_
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn consume(&mut self, type_: TokenType, message: &str) -> Result<Token, LoxError> {
        if self.check(type_) {
            Ok(self.advance())
        } else {
            Err(ParserError::new(self.peek(), message).into())
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().type_ == TokenType::Semicolon {
                return;
            }

            match self.peek().type_ {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {}
            }

            self.advance();
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().type_ == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
