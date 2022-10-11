use crate::expr::Expr;
use crate::lox_error::LoxError;
use crate::token::Literal;
use crate::token_type::TokenType;

fn is_truthy(val: Literal) -> bool {
    match val {
        Literal::None => false,
        Literal::Bool(b) => b,
        _ => true,
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
                        Err(LoxError::new(0, "- on non number type"))
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
                    _ => Err(LoxError::new(0, "- on non number type")),
                },
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
