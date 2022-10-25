use crate::ast::{Expr, Stmt};
use crate::environment::Environment;
use crate::lox_error::{LoxError, ReturnError, RuntimeError};
use crate::native_functions::setup_native_functions;
use crate::token::{Callable, Function, Literal};
use crate::token_type::TokenType;
use std::mem;

fn is_truthy(val: &Literal) -> bool {
    match val {
        Literal::None => false,
        Literal::Bool(b) => *b,
        _ => true,
    }
}

fn is_equal(left: &Literal, right: &Literal) -> bool {
    match (left, right) {
        (Literal::None, Literal::None) => true,
        (Literal::Bool(left), Literal::Bool(right)) => left == right,
        (Literal::Number(left), Literal::Number(right)) => left == right,
        (Literal::String(left), Literal::String(right)) => left == right,
        (_, _) => false,
    }
}

pub struct Interpreter {
    pub globals: Environment,
    pub environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Environment::new();
        setup_native_functions(&mut globals);

        let environment = Environment::from_env(&globals);
        Interpreter {
            globals,
            environment,
        }
    }

    pub fn evaluate(&mut self, expression: &Expr) -> Result<Literal, LoxError> {
        match expression {
            Expr::Assign { name, value } => {
                let value = self.evaluate(value)?;
                self.environment.assign(name, &value)?;
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
                    TokenType::BangEqual => Ok(Literal::Bool(!is_equal(&left, &right))),
                    TokenType::EqualEqual => Ok(Literal::Bool(is_equal(&left, &right))),
                    _ => unreachable!(),
                }
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let callee = self.evaluate(&callee)?;
                let mut values = Vec::new();
                for argument in arguments {
                    values.push(self.evaluate(argument)?);
                }

                match callee {
                    Literal::Callable(c) => {
                        if arguments.len() == c.arity() {
                            c.call(self, &values)
                        } else {
                            let error_msg = format!(
                                "Expected {} arguments but got {}.",
                                c.arity(),
                                arguments.len()
                            );
                            Err(RuntimeError::new(paren, &error_msg).into())
                        }
                    }
                    _ => {
                        Err(RuntimeError::new(paren, "Can only call functions and classes.").into())
                    }
                }
            }
            Expr::Grouping { expression } => self.evaluate(&expression),
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left)?;
                Ok(match operator.type_ {
                    TokenType::Or => {
                        if is_truthy(&left) {
                            left
                        } else {
                            self.evaluate(&right)?
                        }
                    }
                    TokenType::And => {
                        if !is_truthy(&left) {
                            left
                        } else {
                            self.evaluate(&right)?
                        }
                    }
                    _ => unreachable!(),
                })
            }
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
                    TokenType::Bang => Ok(Literal::Bool(!is_truthy(&right))),
                    _ => unreachable!(),
                }
            }
            Expr::Variable { name } => Ok(self.environment.get(name)?),
        }
    }

    pub fn execute(&mut self, statement: &Stmt) -> Result<(), LoxError> {
        match statement {
            Stmt::Block { statements } => {
                self.execute_block(statements, Environment::from_env(&self.environment))?;
            }
            Stmt::Expression { expression } => {
                self.evaluate(&expression)?;
            }
            Stmt::Function { name, params, body } => {
                self.environment.define(
                    name,
                    &Literal::Callable(Callable::Function(Function {
                        closure: self.environment.clone(),
                        params: params.clone(),
                        body: body.clone(),
                    })),
                );
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if is_truthy(&self.evaluate(&condition)?) {
                    self.execute(then_branch)?
                } else if let Some(else_branch) = else_branch {
                    self.execute(else_branch)?
                }
            }
            Stmt::Print { expression } => {
                let value = self.evaluate(&expression)?;
                println!("{}", value);
            }
            Stmt::Return { keyword: _, value } => {
                let value = match value {
                    Some(expr) => self.evaluate(&expr)?,
                    _ => Literal::None,
                };
                return Err(ReturnError { value }.into());
            }
            Stmt::Var { name, initializer } => {
                let value = match initializer {
                    Some(expression) => self.evaluate(&expression)?,
                    None => Literal::None,
                };
                self.environment.define(&name, &value);
            }
            Stmt::While { condition, body } => {
                while is_truthy(&self.evaluate(&condition)?) {
                    self.execute(body)?;
                }
            }
        }
        Ok(())
    }

    pub fn execute_block(
        &mut self,
        statements: &Vec<Stmt>,
        environment: Environment,
    ) -> Result<(), LoxError> {
        let mut env = Environment::from_env(&environment);
        mem::swap(&mut self.environment, &mut env);

        let r = || -> Result<(), LoxError> {
            for statement in statements {
                self.execute(&statement)?;
            }
            Ok(())
        }();

        mem::swap(&mut self.environment, &mut env);

        r
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), LoxError> {
        for statement in statements {
            self.execute(&statement)?;
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
        assert!(is_equal(
            &Literal::Number(-123.0 * 45.67),
            &interpreter.evaluate(&Box::new(expression)).unwrap()
        ));
    }
}
