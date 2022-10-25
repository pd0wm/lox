use crate::token::{Literal, Token};

pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Box<Expr>>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

pub enum Stmt {
    Block {
        statements: Vec<Box<Stmt>>,
    },
    Expression {
        expression: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Print {
        expression: Box<Expr>,
    },
    Var {
        name: Token,
        initializer: Option<Box<Expr>>,
    },
    While {
        condition: Box<Expr>,
        body: Box<Stmt>,
    },
}
