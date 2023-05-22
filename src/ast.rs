#![allow(dead_code)]
use crate::lexer::Token;

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub value: Expression,
    pub name: String,
}

impl LetStatement {
    pub fn new() -> Self {
        LetStatement {
            token: Token::Let,
            name: String::new(),
            value: Expression {
                token: Token::Ident("".to_string()),
                value: String::new(),
            },
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
    ReturnStatement,
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
