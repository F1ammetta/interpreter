#![allow(unused_imports)]
use crate::ast::{Ast, Expression, LetStatement, ReturnStatement, Statement};
#[cfg(test)]
use crate::lexer::Lexer;
use crate::lexer::Token::*;
use crate::parser::Parser;

#[test]
fn parsing() {
    // let input = "
    // let x = 5;
    // let y = 10;
    // let foobar = 838383;
    // ";
    let input = "let x = 5;
    let s = 10;
    
    return 3;
    ";

    let lexer = Lexer::new(input.to_string());
    let mut parser = Parser::new(lexer);

    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            println!("{}", e);
            println!("Parses Errors:");
            for e in parser.errors {
                println!("{}", e);
            }
            panic!();
        }
    };

    assert_eq!(ast.statements.len(), 3);

    let mut expected = vec!["x", "s"];

    for statement in ast.statements {
        match statement {
            Statement::LetStatement(LetStatement { name, value }) => {
                assert_eq!(name, expected.remove(0));
                assert_eq!(value, Expression::default());
            }
            Statement::ReturnStatement(ReturnStatement { value }) => {
                assert_eq!(value, Expression::default());
            }
        }
    }
}

#[test]
fn lexing() {
    let input = "let five = 5;
    let ten = 10;
    let add = fn(x, y) {
        x + y;
    };
    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;

    if (5 < 10) {
        return true;
    } else {
        return false;
    }

    10 == 10;
    10 != 9;

    ten <= 10;

    ten >= 9;

    ";

    let mut lex = Lexer::new(input.to_string());

    let tokens = vec![
        Let,
        Ident("five".to_string()),
        Assign,
        Num("5".to_string()),
        Semicolon,
        Let,
        Ident("ten".to_string()),
        Assign,
        Num("10".to_string()),
        Semicolon,
        Let,
        Ident("add".to_string()),
        Assign,
        Function,
        LParen,
        Ident("x".to_string()),
        Comma,
        Ident("y".to_string()),
        RParen,
        LSquigly,
        Ident("x".to_string()),
        Plus,
        Ident("y".to_string()),
        Semicolon,
        RSquigly,
        Semicolon,
        Let,
        Ident("result".to_string()),
        Assign,
        Ident("add".to_string()),
        LParen,
        Ident("five".to_string()),
        Comma,
        Ident("ten".to_string()),
        RParen,
        Semicolon,
        Bang,
        Minus,
        Slash,
        Asterisk,
        Num("5".to_string()),
        Semicolon,
        Num("5".to_string()),
        LessThan,
        Num("10".to_string()),
        GreaterThan,
        Num("5".to_string()),
        Semicolon,
        If,
        LParen,
        Num("5".to_string()),
        LessThan,
        Num("10".to_string()),
        RParen,
        LSquigly,
        Return,
        True,
        Semicolon,
        RSquigly,
        Else,
        LSquigly,
        Return,
        False,
        Semicolon,
        RSquigly,
        Num("10".to_string()),
        Equal,
        Num("10".to_string()),
        Semicolon,
        Num("10".to_string()),
        NotEqual,
        Num("9".to_string()),
        Semicolon,
        Ident("ten".to_string()),
        LessThanEqual,
        Num("10".to_string()),
        Semicolon,
        Ident("ten".to_string()),
        GreaterThanEqual,
        Num("9".to_string()),
        Semicolon,
        Eof,
    ];

    for token in tokens {
        println!("{:?}", token);
        assert_eq!(token, lex.next_token())
    }
}
