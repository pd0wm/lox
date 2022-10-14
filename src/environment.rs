use crate::lox_error::{LoxError, RuntimeError};
use crate::token::{Literal, Token};
use std::collections::HashMap;

#[derive(Default)]
pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn define(&mut self, name: Token, value: Literal) {
        self.values.insert(name.lexeme, value);
    }

    pub fn get(&self, name: Token) -> Result<Literal, LoxError> {
        match self.values.get(&name.lexeme) {
            Some(literal) => Ok(literal.clone()),
            None => {
                let error_msg = format!("Undefined variable '{}'.", name.lexeme);
                Err(RuntimeError::new(name, &error_msg).into())
            }
        }
    }
}
