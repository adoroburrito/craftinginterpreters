use regex::Regex;
use std::env;
use std::fs;
use std::io::Write;
use std::io::{stdin, stdout};
use std::process;

mod lox;
mod scanner;

use crate::lox::LoxInterpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let length = args.len();
    if length > 2 {
        println!("Usage: rlox [script]");
        process::exit(64);
    }

    let mut lox_instance = lox::Lox { had_error: false };

    if length == 2 {
        run_file(&args[1], &mut lox_instance);
    } else {
        run_prompt(&mut lox_instance);
    }
}

fn run_file(path: &String, lox_instance: &mut lox::Lox) {
    println!("Running file \"{path}\"");
    let content_string = fs::read_to_string(path);
    let content_string = match content_string {
        Ok(content) => content,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    lox_instance.run(&content_string);

    if lox_instance.had_error {
        process::exit(65);
    }
}

fn run_prompt(lox_instance: &mut lox::Lox) {
    let re = Regex::new(r"\s+").unwrap();

    loop {
        let mut s = String::new();
        print!("> ");

        match stdout().flush() {
            Ok(_all_good) => {}
            Err(error) => panic!("Failed to flush stdout: {:?}", error),
        };

        match stdin().read_line(&mut s) {
            Ok(_goes_into_input_above) => {}
            Err(error) => panic!("Failed interpreting input: {:?}", error),
        }

        let string = &s.trim().to_string();
        let result = re.replace_all(string, " ");
        lox_instance.run(&result.to_string());
        lox_instance.had_error = false;
    }
}
