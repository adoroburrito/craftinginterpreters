use crate::scanner::token::TokenTrait;
use crate::scanner::{create_scanner, ScannerActions};

pub struct Lox {
    pub had_error: bool,
}

pub trait LoxInterpreter {
    fn run(&mut self, content: &str);
    fn error(&mut self, line: u8, message: &str);
    fn report(&mut self, line: u8, where_at: &str, message: &str);
}

impl LoxInterpreter for Lox {
    fn run(&mut self, content: &str) {
        println!("Tokens:");
        let mut scanner_instance = create_scanner(self, content.to_string());
        let tokens = scanner_instance.scan_tokens();

        for token in tokens {
            let token_string = token.get_string();
            println!("\t- {token_string}");
        }
    }

    fn report(&mut self, line: u8, where_at: &str, message: &str) {
        eprintln!("[line {line}] Error{where_at}: {message}");
        self.had_error = true;
    }

    fn error(&mut self, line: u8, message: &str) {
        self.report(line, &"".to_owned(), message);
    }
}
