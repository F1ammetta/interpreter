mod ast;
mod lexer;
mod parser;
mod test;

use lexer::Lexer;
use parser::Parser;
use std::io::Write;

fn main() {
    println!("Welcome to my Rusty Interpreter!");
    repl();
}

fn repl() {
    loop {
        let mut s = String::new();
        print!(">> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut s).unwrap();
        let input = s.trim().to_string();
        if input == "exit" {
            break;
        }
        let mut parser = Parser::new(Lexer::new(input));
        let ast = parser.parse();
        match ast {
            Ok(ast) => println!("{:?}", ast),
            Err(_) => {
                for err in parser.errors {
                    println!("Parser Error: {}", err);
                }
            }
        }
    }
}
