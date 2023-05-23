#![allow(dead_code)]
use Token::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token {
    Illegal,
    Eof,
    Ident(String),
    Num(String),
    Assign,
    Plus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LSquigly,
    RSquigly,
    Function,
    Let,
    Equal,
    NotEqual,
    Bang,
    Minus,
    Slash,
    Asterisk,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    LBracket,
    RBracket,
    If,
    Else,
    True,
    False,
    Return,
    And,
    Or,
    Xor,
}

impl Default for Token {
    fn default() -> Self {
        Ident(String::new())
    }
}

pub struct Lexer {
    position: usize,
    read_position: usize,
    input: Vec<u8>,
    ch: u8,
}

fn is_letter(ch: u8) -> bool {
    b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_'
}

fn is_number(ch: u8) -> bool {
    b'0' <= ch && ch <= b'9' || ch == b'.'
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lex = Lexer {
            position: 0,
            read_position: 0,
            input: input.into_bytes(),
            ch: b'\0',
        };
        lex.read_char();
        lex
    }

    fn consume_everything(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.consume_everything();
        match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    return Equal;
                }
                self.read_char();
                Assign
            }
            b'&' => {
                if self.peek_char() == b'&' {
                    self.read_char();
                    self.read_char();
                    return And;
                }
                self.read_char();
                todo!("Bitwise And");
            }
            b'|' => {
                if self.peek_char() == b'|' {
                    self.read_char();
                    self.read_char();
                    return Or;
                }
                self.read_char();
                todo!("Bitwise Or");
            }
            b'^' => {
                self.read_char();
                Xor
            }
            b'+' => {
                self.read_char();
                Plus
            }
            b',' => {
                self.read_char();
                Comma
            }
            b';' => {
                self.read_char();
                Semicolon
            }
            b'(' => {
                self.read_char();
                LParen
            }
            b')' => {
                self.read_char();
                RParen
            }
            b'{' => {
                self.read_char();
                LSquigly
            }
            b'}' => {
                self.read_char();
                RSquigly
            }
            b'-' => {
                self.read_char();
                Minus
            }
            b'/' => {
                self.read_char();
                Slash
            }
            b'*' => {
                self.read_char();
                Asterisk
            }
            b'<' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    return LessThanEqual;
                }

                self.read_char();
                LessThan
            }
            b'>' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    return GreaterThanEqual;
                }
                self.read_char();
                GreaterThan
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    self.read_char();
                    return NotEqual;
                }
                self.read_char();
                Bang
            }
            b'[' => {
                self.read_char();
                LBracket
            }
            b']' => {
                self.read_char();
                RBracket
            }
            b'\0' => Eof,
            _ => {
                if is_letter(self.ch) {
                    let ident = self.read_identifier();
                    match ident.as_str() {
                        "fn" => Function,
                        "let" => Let,
                        "true" => True,
                        "false" => False,
                        "if" => If,
                        "else" => Else,
                        "return" => Return,
                        _ => Ident(ident),
                    }
                } else if is_number(self.ch) {
                    let number = self.read_number();
                    Num(number)
                } else {
                    Illegal
                }
            }
        }
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return b'\0';
        }
        self.input[self.read_position]
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while is_number(self.ch) {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[position..self.position]).to_string()
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[position..self.position]).to_string()
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            tokens.push(token.clone());
            if token == Token::Eof {
                break;
            }
        }
        tokens
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = b'\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
}
