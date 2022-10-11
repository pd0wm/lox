use crate::expr::Expr;
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

fn is_equal(left: Literal, right: Literal) -> bool{
    match (left, right) {
        (Literal::None, Literal::None) => true,
        (Literal::Bool(left), Literal::Bool(right)) => left == right,
        (Literal::Number(left), Literal::Number(right)) => left == right,
        (Literal::String(left), Literal::String(right)) => left == right,
        (Literal::None, _) => false, // redundant?
        (_, _) => false,
    }
}

fn evaluate(expression: Box<Expr>) -> Result<Literal, LoxError> {
    match *expression {
        Expr::Literal { value } => Ok(value),
        Expr::Grouping { expression } => evaluate(expression),
        Expr::Unary { operator, right } => {
            let right = evaluate(right)?;
            match operator.type_ {
                TokenType::Minus => {
                    if let Literal::Number(right) = right {
                        Ok(Literal::Number(-right))
                    } else {
                        Err(LoxError::Runtime(RuntimeError::new(operator, "Operand must be a number.")))
                    }
                }
                TokenType::Bang => Ok(Literal::Bool(!is_truthy(right))),
                _ => unreachable!(),
            }
        }
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let left = evaluate(left)?;
            let right = evaluate(right)?;

            match operator.type_ {
                TokenType::Minus => match (left, right) {
                    (Literal::Number(left), Literal::Number(right)) => {
                        Ok(Literal::Number(left - right))
                    }
                    _ => Err(LoxError::Runtime(RuntimeError::new(operator, "Operands must be numbers."))),
                },
                TokenType::Slash => match (left, right) {
                    (Literal::Number(left), Literal::Number(right)) => {
                        Ok(Literal::Number(left / right))
                    }
                    _ => Err(LoxError::Runtime(RuntimeError::new(operator, "Operands must be numbers."))),
                },
                TokenType::Star => match (left, right) {
                    (Literal::Number(left), Literal::Number(right)) => {
                        Ok(Literal::Number(left * right))
                    }
                    _ => Err(LoxError::Runtime(RuntimeError::new(operator, "Operands must be numbers."))),
                },
                TokenType::Plus => match (left, right) {
                    (Literal::Number(left), Literal::Number(right)) => {
                        Ok(Literal::Number(left + right))
                    }
                    (Literal::String(left), Literal::String(right)) => {
                        Ok(Literal::String(left + &right))
                    }
                    _ => Err(LoxError::Runtime(RuntimeError::new(operator, "Operands must be two numbers or two strings."))),
                },
                TokenType::Greater => match (left, right) {
                    (Literal::Number(left), Literal::Number(right)) => {
                        Ok(Literal::Bool(left > right))
                    }
                    _ => Err(LoxError::Runtime(RuntimeError::new(operator, "Operands must be numbers."))),
                },
                TokenType::GreaterEqual => match (left, right) {
                    (Literal::Number(left), Literal::Number(right)) => {
                        Ok(Literal::Bool(left >= right))
                    }
                    _ => Err(LoxError::Runtime(RuntimeError::new(operator, "Operands must be numbers."))),
                },
                TokenType::Less => match (left, right) {
                    (Literal::Number(left), Literal::Number(right)) => {
                        Ok(Literal::Bool(left < right))
                    }
                    _ => Err(LoxError::Runtime(RuntimeError::new(operator, "Operands must be numbers."))),
                },
                TokenType::LessEqual => match (left, right) {
                    (Literal::Number(left), Literal::Number(right)) => {
                        Ok(Literal::Bool(left <= right))
                    }
                    _ => Err(LoxError::Runtime(RuntimeError::new(operator, "Operands must be numbers."))),
                },
                TokenType::BangEqual => Ok(Literal::Bool(!is_equal(left, right))),
                TokenType::EqualEqual => Ok(Literal::Bool(is_equal(left, right))),
                _ => unreachable!(),
            }
        }
    }
}

pub fn interpret(expression: Box<Expr>) -> Result<(), LoxError> {
    let value = evaluate(expression)?;
    println!("{}", value);

    Ok(())
}
