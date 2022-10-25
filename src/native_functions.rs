use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::lox_error::LoxError;
use crate::token::{Callable, Literal, NativeFunction, Token};
use crate::token_type::TokenType;

use std::time::{SystemTime, UNIX_EPOCH};

fn clock_fn(_interpreter: &Interpreter, _arguments: &Vec<Literal>) -> Result<Literal, LoxError> {
    let now = SystemTime::now();
    let secs = now.duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
    Ok(Literal::Number(secs))
}

pub fn setup_native_functions(environment: &mut Environment) {
    environment.define(
        &Token {
            type_: TokenType::Fun,
            lexeme: "clock".to_string(),
            literal: None,
            line: 0,
        },
        &Literal::Callable(Callable::NativeFunction(NativeFunction {
            arity: 0,
            closure: clock_fn,
        })),
    );
}
