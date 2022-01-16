use std::env;
use std::fs;
use std::process;
use std::io::Write;
use std::io::{stdin, stdout};

use regex::Regex;

struct Lox {
    had_error: bool
}

trait LoxInterpreter {
    fn run(&mut self, content: &String);
    fn error(&mut self, line: u8, message: &String);
    fn report(&mut self, line: u8, where_at: &String, message: &String);
}

impl LoxInterpreter for Lox {
    fn run(&mut self, content: &String) {
        println!("Tokens:");
        let tokens: Vec<&str> = content.split(" ").collect();
        for token in tokens {
            println!("\t- {token}");
        }
    }

    fn report(&mut self, line: u8, where_at: &String, message: &String){
        eprintln!("[line {line}] Error{where_at}: {message}");
        self.had_error = true;
    }

    fn error(&mut self, line: u8, message: &String) {
        self.report(line, &"".to_owned(), message);
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let length = args.len();
    if length > 2 {
        println!("Usage: rlox [script]");
        process::exit(0x0100)
    } 

    let mut lox = Lox {
        had_error: false
    };
    
    if length == 2 {
        run_file(&args[1], &mut lox);
    } else {
        run_prompt(&mut lox);
    }
}

fn run_file(path: &String, lox_instance: &mut Lox){
    println!("Running file \"{path}\"");
    let content_string = fs::read_to_string(path);
    let content_string = match content_string {
        Ok(content) => content,
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };

    lox_instance.run(&content_string);
}

fn run_prompt(lox_instance: &mut Lox) {

    let re = Regex::new(r"\s+").unwrap();

    loop {
        let mut s = String::new();
        print!("> ");

        match stdout().flush() {
            Ok(_all_good) => {},
            Err(error) => panic!("Failed to flush stdout: {:?}", error)
        };
        
        match stdin().read_line(&mut s) {
            Ok(_goes_into_input_above) => {},
            Err(error) => panic!("Failed interpreting input: {:?}", error)
        }

        let string = &s.trim().to_string();
        let result = re.replace_all(string, " ");
        lox_instance.run(&result.to_string());
    }
}