use std::mem;

use crate::ast::{Expr, Stmt};
use crate::environment::Environment;
use crate::lox_error::{LoxError, RuntimeError};
use crate::token::Literal;
use crate::token_type::TokenType;

fn is_truthy(val: Literal) -> bool {
    match val {
        Literal::None => false,
        Literal::Bool(b) => b,
        _ => true,
    }
}

fn is_equal(left: Literal, right: Literal) -> bool {
    match (left, right) {
        (Literal::None, Literal::None) => true,
        (Literal::Bool(left), Literal::Bool(right)) => left == right,
        (Literal::Number(left), Literal::Number(right)) => left == right,
        (Literal::String(left), Literal::String(right)) => left == right,
        (_, _) => false,
    }
}

#[derive(Default)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn evaluate(&mut self, expression: Box<Expr>) -> Result<Literal, LoxError> {
        match *expression {
            Expr::Assign { name, value } => {
                let value = self.evaluate(value)?;
                self.environment.assign(name, value.clone())?;
                Ok(value)
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;

                match operator.type_ {
                    TokenType::Minus => match (left, right) {
                        (Literal::Number(left), Literal::Number(right)) => {
                            Ok(Literal::Number(left - right))
                        }
                        _ => Err(RuntimeError::new(operator, "Operands must be numbers.").into()),
                    },
                    TokenType::Slash => match (left, right) {
                        (Literal::Number(left), Literal::Number(right)) => {
                            Ok(Literal::Number(left / right))
                        }
                        _ => Err(RuntimeError::new(operator, "Operands must be numbers.").into()),
                    },
                    TokenType::Star => match (left, right) {
                        (Literal::Number(left), Literal::Number(right)) => {
                            Ok(Literal::Number(left * right))
                        }
                        _ => Err(RuntimeError::new(operator, "Operands must be numbers.").into()),
                    },
                    TokenType::Plus => match (left, right) {
                        (Literal::Number(left), Literal::Number(right)) => {
                            Ok(Literal::Number(left + right))
                        }
                        (Literal::String(left), Literal::String(right)) => {
                            Ok(Literal::String(left + &right))
                        }
                        _ => Err(RuntimeError::new(
                            operator,
                            "Operands must be two numbers or two strings.",
                        )
                        .into()),
                    },
                    TokenType::Greater => match (left, right) {
                        (Literal::Number(left), Literal::Number(right)) => {
                            Ok(Literal::Bool(left > right))
                        }
                        _ => Err(RuntimeError::new(operator, "Operands must be numbers.").into()),
                    },
                    TokenType::GreaterEqual => match (left, right) {
                        (Literal::Number(left), Literal::Number(right)) => {
                            Ok(Literal::Bool(left >= right))
                        }
                        _ => Err(RuntimeError::new(operator, "Operands must be numbers.").into()),
                    },
                    TokenType::Less => match (left, right) {
                        (Literal::Number(left), Literal::Number(right)) => {
                            Ok(Literal::Bool(left < right))
                        }
                        _ => Err(RuntimeError::new(operator, "Operands must be numbers.").into()),
                    },
                    TokenType::LessEqual => match (left, right) {
                        (Literal::Number(left), Literal::Number(right)) => {
                            Ok(Literal::Bool(left <= right))
                        }
                        _ => Err(RuntimeError::new(operator, "Operands must be numbers.").into()),
                    },
                    TokenType::BangEqual => Ok(Literal::Bool(!is_equal(left, right))),
                    TokenType::EqualEqual => Ok(Literal::Bool(is_equal(left, right))),
                    _ => unreachable!(),
                }
            }
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Literal { value } => Ok(value),
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right)?;
                match operator.type_ {
                    TokenType::Minus => {
                        if let Literal::Number(right) = right {
                            Ok(Literal::Number(-right))
                        } else {
                            Err(RuntimeError::new(operator, "Operand must be a number.").into())
                        }
                    }
                    TokenType::Bang => Ok(Literal::Bool(!is_truthy(right))),
                    _ => unreachable!(),
                }
            }
            Expr::Variable { name } => Ok(self.environment.get(name)?),
        }
    }

    pub fn execute(&mut self, statement: Stmt) -> Result<(), LoxError> {
        match statement {
            Stmt::Block { statements } => {
                let mut env = Environment::new(&self.environment);
                mem::swap(&mut self.environment, &mut env);

                for statement in statements {
                    self.execute(*statement)?;
                }

                mem::swap(&mut self.environment, &mut env);
            }
            Stmt::Expression { expression } => {
                self.evaluate(expression)?;
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if is_truthy(self.evaluate(condition)?) {
                    self.execute(*then_branch)?
                } else if else_branch.is_some() {
                    self.execute(*else_branch.unwrap())?
                }
            }
            Stmt::Print { expression } => {
                let value = self.evaluate(expression)?;
                println!("{}", value);
            }
            Stmt::Var { name, initializer } => {
                let value = match initializer {
                    Some(expression) => self.evaluate(expression)?,
                    None => Literal::None,
                };
                self.environment.define(name, value);
            }
        }
        Ok(())
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), LoxError> {
        for statement in statements {
            self.execute(statement)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{Literal, Token};
    use crate::token_type::TokenType;

    use super::*;

    #[test]
    fn test_evaluate() {
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

        let mut interpreter = Interpreter::new();
        assert_eq!(
            Literal::Number(-123.0 * 45.67),
            interpreter.evaluate(Box::new(expression)).unwrap()
        );
    }
}
