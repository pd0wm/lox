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

fn is_equal(left: Literal, right: Literal) -> bool {
    match (left, right) {
        (Literal::None, Literal::None) => true,
        (Literal::Bool(left), Literal::Bool(right)) => left == right,
        (Literal::Number(left), Literal::Number(right)) => left == right,
        (Literal::String(left), Literal::String(right)) => left == right,
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
                        Err(RuntimeError::new(operator, "Operand must be a number.").into())
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
    }
}

pub fn interpret(expression: Box<Expr>) -> Result<(), LoxError> {
    let value = evaluate(expression)?;
    println!("{}", value);

    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::token::{Token, Literal};
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

        assert_eq!(Literal::Number(-123.0 * 45.67), evaluate(Box::new(expression)).unwrap());
    }
}
