use std::fmt;

use crate::token::{Literal, Token};

pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

pub enum Stmt {
    Expression {
        expression: Box<Expr>,
    },
    Print {
        expression: Box<Expr>,
    },
    Var {
        name: Token,
        initializer: Option<Box<Expr>>,
    },
}

fn parenthesize(f: &mut fmt::Formatter, name: &str, exprs: &[&Box<Expr>]) -> fmt::Result {
    write!(f, "({}", name)?;
    for expr in exprs {
        write!(f, " {}", expr)?;
    }
    write!(f, ")")
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Assign { name, value } => write!(f, "var {} = {}", name, value),
            Expr::Binary {
                left,
                operator,
                right,
            } => parenthesize(f, &operator.lexeme, &[left, right]),
            Expr::Grouping { expression } => parenthesize(f, "group", &[expression]),
            Expr::Literal { value } => write!(f, "{}", value),
            Expr::Unary { operator, right } => parenthesize(f, &operator.lexeme, &[right]),
            Expr::Variable { name } => write!(f, "var {}", name),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token_type::TokenType;

    use super::*;

    #[test]
    fn test_fmt() {
        // Example from 5.4
        let expression = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token::new(TokenType::Minus, "-", None, 1),
                right: Box::new(Expr::Literal {
                    value: Literal::Number(123.0),
                }),
            }),
            operator: Token::new(TokenType::Star, "*", None, 1),
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal {
                    value: Literal::Number(45.67),
                }),
            }),
        };

        assert_eq!("(* (- 123) (group 45.67))", expression.to_string());
    }
}
