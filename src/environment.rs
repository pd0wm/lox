use crate::lox_error::{LoxError, RuntimeError};
use crate::token::{Literal, Token};
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;

#[derive(Default)]
struct EnvironmentValues {
    values: HashMap<String, Literal>,
    enclosing: Option<Rc<RefCell<EnvironmentValues>>>,
}

impl EnvironmentValues {
    pub fn new() -> Rc<RefCell<EnvironmentValues>> {
        Rc::new(RefCell::new(EnvironmentValues {
            enclosing: None,
            ..Default::default()
        }))
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
                Some(enclosing) => enclosing.borrow_mut().assign(name, &value),
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
                Some(enclosing) => enclosing.borrow().get(&name),
                _ => {
                    let error_msg = format!("Undefined variable '{}'.", name.lexeme);
                    Err(RuntimeError::new(name, &error_msg).into())
                }
            },
        }
    }
}

#[derive(Default)]
pub struct Environment {
    head: Rc<RefCell<EnvironmentValues>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            head: EnvironmentValues::new(),
        }
    }

    pub fn push(&mut self) {
        let mut new = EnvironmentValues::new();
        mem::swap(&mut self.head, &mut new);
        self.head.borrow_mut().enclosing = Some(new);
    }

    pub fn pop(&mut self) {
        let new = mem::take(&mut self.head.borrow_mut().enclosing); // Replaces head.enclosing with None
        self.head = new.unwrap();
    }

    pub fn define(&mut self, name: &Token, value: &Literal) {
        self.head.borrow_mut().define(name, value)
    }

    pub fn assign(&mut self, name: &Token, value: &Literal) -> Result<(), LoxError> {
        self.head.borrow_mut().assign(name, value)
    }

    pub fn get(&self, name: &Token) -> Result<Literal, LoxError> {
        self.head.borrow_mut().get(name)
    }
}
