#![allow(dead_code)]
use crate::lexer::Token;

#[derive(Debug)]
pub struct LetStatement {
    pub value: Expression,
    pub name: String,
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub value: Expression,
}

impl ReturnStatement {
    pub fn new() -> Self {
        ReturnStatement {
            value: Expression::default(),
        }
    }
}

impl LetStatement {
    pub fn new() -> Self {
        LetStatement {
            name: String::new(),
            value: Expression::default(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct Expression {
    pub token: Token,
    pub value: String,
}

#[derive(Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
}

#[derive(Debug)]
pub struct Ast {
    pub statements: Vec<Statement>,
}

impl Ast {
    pub fn new() -> Self {
        Ast {
            statements: Vec::new(),
        }
    }
}
