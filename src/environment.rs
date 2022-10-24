use crate::lox_error::{LoxError, RuntimeError};
use crate::token::{Literal, Token};
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Environment {
    values: HashMap<String, Literal>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(enclosing: &Environment) -> Self {
        Self {
            enclosing: Some(Box::new(enclosing.clone())),
            ..Default::default()
        }
    }

    pub fn define(&mut self, name: &Token, value: &Literal) {
        self.values.insert(name.lexeme.clone(), value.clone());
    }

    pub fn assign(&mut self, name: &Token, value: &Literal) -> Result<(), LoxError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());
            Ok(())
        } else {
            match &mut self.enclosing {
                Some(enclosing) => enclosing.assign(name, &value),
                _ => {
                    let error_msg = format!("Undefined variable '{}'.", name.lexeme);
                    Err(RuntimeError::new(name, &error_msg).into())
                }
            }
        }
    }

    pub fn get(&self, name: &Token) -> Result<Literal, LoxError> {
        match self.values.get(&name.lexeme) {
            Some(literal) => Ok(literal.clone()),
            None => match &self.enclosing {
                Some(enclosing) => enclosing.get(&name),
                _ => {
                    let error_msg = format!("Undefined variable '{}'.", name.lexeme);
                    Err(RuntimeError::new(name, &error_msg).into())
                }
            },
        }
    }
}
