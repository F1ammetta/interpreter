#![allow(dead_code)]
use crate::ast::{Ast, LetStatement, ReturnStatement, Statement};
use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    pub errors: Vec<std::io::Error>,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
            peek_token,
            errors: Vec::new(),
        }
    }
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_statement(&mut self) -> Result<Statement, std::io::Error> {
        match self.current_token {
            Token::Let => Ok(self.parse_let_statement()),
            Token::Return => Ok(self.parse_return_statement()),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid token",
            )),
        }
    }

    fn parse_return_statement(&mut self) -> Statement {
        let mut statement = ReturnStatement::new();
        self.next_token();

        while self.current_token != Token::Semicolon {
            self.next_token();
        }

        Statement::ReturnStatement(statement)
    }

    fn parse_let_statement(&mut self) -> Statement {
        let mut statement = LetStatement::new();

        match self.peek_token {
            Token::Ident(ref name) => {
                statement.name = name.clone();
                self.next_token();
            }
            _ => {
                self.errors.push(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Expected identifier, got {:?}", self.peek_token),
                ));
                self.next_token();
            }
        }

        match self.peek_token {
            Token::Assign => {
                self.next_token();
            }
            _ => {
                self.errors.push(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Expected Assign, got {:?}", self.peek_token),
                ));
                self.next_token();
            }
        }

        while self.current_token != Token::Semicolon && self.current_token != Token::Eof {
            self.next_token();
        }

        Statement::LetStatement(statement)
    }

    pub fn parse(&mut self) -> Result<Ast, std::io::Error> {
        let mut ast = Ast::new();

        while self.current_token != Token::Eof {
            let statement = match self.parse_statement() {
                Ok(statement) => statement,
                Err(e) => return Err(e),
            };
            ast.statements.push(statement);
            self.next_token();
        }

        if self.errors.len() > 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("{} errors found", self.errors.len()),
            ));
        }

        Ok(ast)
    }
}
