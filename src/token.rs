use crate::interpreter::Interpreter;
use crate::lox_error::LoxError;
use crate::token_type::TokenType;
use std::fmt;

#[derive(Clone)]
pub enum Literal {
    None,
    Bool(bool),
    Callable(Callable),
    String(String),
    Number(f64),
}

#[derive(Clone)]
pub enum Callable {
    Function(Function),
    NativeFunction(NativeFunction),
}

// Use trait? Breaks Clone on Literal
impl Callable {
    pub fn arity(&self) -> usize {
        match self {
            Callable::Function(f) => f.arity,
            Callable::NativeFunction(f) => f.arity,
        }
    }

    pub fn call(
        &self,
        interpreter: &Interpreter,
        arguments: &Vec<Literal>,
    ) -> Result<Literal, LoxError> {
        match self {
            Callable::Function(f) => f.call(interpreter, arguments),
            Callable::NativeFunction(f) => f.call(interpreter, arguments),
        }
    }
}

#[derive(Clone)]
pub struct NativeFunction {
    pub arity: usize,
    pub closure: fn(&Interpreter, &Vec<Literal>) -> Result<Literal, LoxError>,
}

impl NativeFunction {
    pub fn call(
        &self,
        interpreter: &Interpreter,
        arguments: &Vec<Literal>,
    ) -> Result<Literal, LoxError> {
        (self.closure)(interpreter, arguments)
    }
}

#[derive(Clone)]
pub struct Function {
    pub arity: usize,
}

impl Function {
    pub fn call(
        &self,
        _interpreter: &Interpreter,
        _arguments: &Vec<Literal>,
    ) -> Result<Literal, LoxError> {
        Ok(Literal::None)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::None => write!(f, "nil"),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Callable(c) => write!(f, "callable({})", c.arity()),
            Literal::String(t) => write!(f, "{}", t),
            Literal::Number(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Clone)]
pub struct Token {
    pub type_: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(type_: TokenType, lexeme: &str, literal: Option<Literal>, line: usize) -> Self {
        Self {
            type_,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(literal) = &self.literal {
            write!(f, "{} {} {}", self.type_, self.lexeme, literal)
        } else {
            write!(f, "{} {} None", self.type_, self.lexeme,)
        }
    }
}
