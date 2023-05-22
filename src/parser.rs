#![allow(dead_code)]
use crate::ast::{Ast, LetStatement, Statement};
use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
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
        }
    }
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_statement(&mut self) -> Result<Statement, std::io::Error> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            // Token::Return => self.parse_return_statement(),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid token",
            )),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, std::io::Error> {
        let mut statement = LetStatement::new();
        statement.token = self.current_token.clone();

        match self.peek_token {
            Token::Ident(_) => {
                self.next_token();
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid token",
                ))
            }
        }

        statement.name = match self.current_token {
            Token::Ident(ref name) => name.clone(),
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid token",
                ))
            }
        };

        match self.peek_token {
            Token::Assign => {
                self.next_token();
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid token",
                ))
            }
        }

        while self.current_token != Token::Semicolon {
            self.next_token();
        }

        Ok(Statement::LetStatement(statement))
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

        Ok(ast)
    }
}
