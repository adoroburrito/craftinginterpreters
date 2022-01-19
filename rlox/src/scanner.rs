use token::Token;
use token_type::Token_Type;

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u8,
    current: u8,
    line: u8,
    lox_instance: Lox
}

trait Scanner_Actions {
    fn scan_tokens(&mut self) -> Vec<Token>;
    fn is_at_end(&self) -> bool;
    fn scan_token(&mut self);
    fn advance(&self) -> char;
    fn add_token(&mut self);
    fn match_next(&mut self, expected: char) -> bool;
    fn peek(&self) -> char;
}

impl Scanner_Actions for Scanner {
    fn scan_tokens(&mut self) -> Vec<Token> {
        while !&self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            tokentype: Token_Type.EOF,
            lexeme: "",
            literal: (),
            line: self.line
        });

        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.chars().count()
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(Token_Type.LEFT_PAREN),
            ')' => self.add_token(Token_Type.RIGHT_PAREN),
            '{' => self.add_token(Token_Type.LEFT_BRACE),
            '}' => self.add_token(Token_Type.RIGHT_BRACE),
            ',' => self.add_token(Token_Type.COMMA),
            '.' => self.add_token(Token_Type.DOT),
            '-' => self.add_token(Token_Type.MINUS),
            '+' => self.add_token(Token_Type.PLUS),
            ';' => self.add_token(Token_Type.SEMICOLON),
            '*' => self.add_token(Token_Type.STAR),
            '!' => self.add_token(if self.match_next('=') { Token_Type.BAND_EQUAL } else { Token_Type.BANG }),
            '=' => self.add_token(if self.match_next('=') { Token_Type.EQUAL_EQUAL } else { Token_Type.EQUAL }),
            '<' => self.add_token(if self.match_next('=') { Token_Type.LESS_EQUAL } else { Token_Type.LESS }),
            '>' => self.add_token(if self.match_next('=') { Token_Type.GREATER_EQUAL } else { Token_Type.GREATER }),
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Token_Type.SLASH)
                }

            },
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            _ => self.lox_instance.error(self.line, "Unexpected character")
        }
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false
        }

        let source_chars = self.source.chars();
        let current_char = source_chars.nth(self.current).unwrap();

        if current_char != expected {
            return false
        }

        self.current += 1;

        true
    }

    fn peek(&self) -> bool {
        if(self.is_at_end) {
            return '\0'
        }

        return self.source.chars().nth(self.current).unwrap();
    }
}

pub fn create_scanner(lox_instance: &Lox) -> Scanner {
    Scanner {
        source: String::from(""),
        tokens: Vec::new(),
        start: 0,
        current: 0,
        line: 1,
        lox_instance
    }
}
