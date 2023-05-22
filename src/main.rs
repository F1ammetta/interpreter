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
        let mut parser = Parser::new(Lexer::new(input));
        let ast = parser.parse();
        println!("{:?}", ast);
        println!("Errors: {:?}", parser.errors);
    }
}
