pub mod token;
mod token_type;

use crate::lox::{Lox, LoxInterpreter};
use token::Token;
use token_type::TokenType;

pub struct Scanner<'a> {
    source: String,
    tokens: Vec<Token>,
    start: u8,
    current: u8,
    line: u8,
    lox_instance: &'a mut Lox,
}

pub trait ScannerActions {
    fn scan_tokens(&mut self) -> &Vec<Token>;
    fn is_at_end(&self) -> bool;
    fn scan_token(&mut self);
    fn string(&mut self);
    fn advance(&mut self) -> char;
    fn add_token(&mut self, tokentype: TokenType);
    fn add_token_with_literal(&mut self, tokentype: TokenType, literal: String);
    fn match_next(&mut self, expected: char) -> bool;
    fn peek(&self) -> char;
}

impl ScannerActions for Scanner<'_> {
    fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let new_token = Token {
            tokentype: TokenType::Eof,
            lexeme: "".to_string(),
            literal: "".to_string(),
            line: self.line,
        };

        self.tokens.push(new_token);

        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        usize::from(self.current) >= self.source.chars().count()
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        let match_equals: bool = self.match_next('=');
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
            '!' => self.add_token(if match_equals {
                TokenType::BangEqual
            } else {
                TokenType::Bang
            }),
            '=' => self.add_token(if match_equals {
                TokenType::EqualEqual
            } else {
                TokenType::Equal
            }),
            '<' => self.add_token(if match_equals {
                TokenType::LessEqual
            } else {
                TokenType::Less
            }),
            '>' => self.add_token(if match_equals {
                TokenType::GreaterEqual
            } else {
                TokenType::Greater
            }),
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => self
                .lox_instance
                .error(self.line, &String::from("Unexpected character")),
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            };
            self.advance();
        }

        if self.is_at_end() {
            self.lox_instance
                .error(self.line, &"Unterminated string.".to_string());
            return;
        }

        // The closing "
        self.advance();

        // Trim the surrounding quotes
        let value: String = self
            .source
            .chars()
            .skip((self.start + 1).into())
            .take(((self.current - self.start) - 1).into())
            .collect();
        self.add_token_with_literal(TokenType::StringText, value);
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let mut source_chars = self.source.chars();
        let current_char = &source_chars.nth(self.current.into()).unwrap();

        if current_char != &expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self.source.chars().nth(self.current.into()).unwrap();
    }

    fn advance(&mut self) -> char {
        self.current += 1;

        match self.source.chars().nth(self.current.into()) {
            Some(value) => value,
            None => ' ',
        }
    }

    fn add_token(&mut self, tokentype: TokenType) {
        let text = self
            .source
            .chars()
            .skip(self.start.into())
            .take((self.current - self.start).into())
            .collect();
        self.tokens.push(Token {
            tokentype,
            lexeme: text,
            literal: String::from(""),
            line: self.line,
        });
    }

    fn add_token_with_literal(&mut self, tokentype: TokenType, literal: String) {
        let text = self
            .source
            .chars()
            .skip(self.start.into())
            .take((self.current - self.start).into())
            .collect();
        self.tokens.push(Token {
            tokentype,
            lexeme: text,
            literal,
            line: self.line,
        });
    }
}

pub fn create_scanner(lox_instance: &mut Lox, content: String) -> Scanner {
    Scanner {
        source: content,
        tokens: Vec::new(),
        start: 0,
        current: 0,
        line: 1,
        lox_instance,
    }
}
