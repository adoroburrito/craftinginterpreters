pub mod token;
mod token_type;

use token::Token;
use token_type::TokenType;
use crate::lox::{Lox, LoxInterpreter};

pub struct Scanner<'a> {
    source: String,
    tokens: Vec<Token>,
    start: u8,
    current: u8,
    line: u8,
    lox_instance: &'a Lox
}

pub trait ScannerActions {
    fn scan_tokens(&mut self) -> Vec<Token>;
    fn is_at_end(&self) -> bool;
    fn scan_token(&mut self);
    fn advance(&self) -> char;
    fn add_token(&mut self, token: TokenType);
    fn match_next(&mut self, expected: char) -> bool;
    fn peek(&self) -> char;
}

impl ScannerActions for Scanner<'_> {
    fn scan_tokens(&mut self) -> Vec<Token> {
        while !&self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            tokentype: TokenType::Eof,
            lexeme: "".to_string(),
            literal: (),
            line: self.line
        });

        self.tokens
    }

    fn is_at_end(&self) -> bool {
        usize::from(self.current) >= self.source.chars().count()
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => self.add_token(if self.match_next('=') { TokenType::BangEqual } else { TokenType::Bang }),
            '=' => self.add_token(if self.match_next('=') { TokenType::EqualEqual } else { TokenType::Equal }),
            '<' => self.add_token(if self.match_next('=') { TokenType::LessEqual } else { TokenType::Less }),
            '>' => self.add_token(if self.match_next('=') { TokenType::GreaterEqual } else { TokenType::Greater }),
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }

            },
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            _ => self.lox_instance.error(self.line, &String::from("Unexpected character"))
        }
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false
        }

        let source_chars = self.source.chars();
        let current_char = source_chars.nth(self.current.into()).unwrap();

        if current_char != expected {
            return false
        }

        self.current += 1;

        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0'
        }

        return self.source.chars().nth(self.current.into()).unwrap();
    }
}

pub fn create_scanner(lox_instance: &Lox, content: String) -> Scanner {
    Scanner {
        source: content,
        tokens: Vec::new(),
        start: 0,
        current: 0,
        line: 1,
        lox_instance
    }
}
