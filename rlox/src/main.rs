use std::env;
use std::fs;
use std::io::Write;
use std::io::{stdin, stdout};

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let length = args.len();
    if length > 2 {
        println!("Usage: rlox [script]")
    } else if length == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &String){
    println!("Running file \"{path}\"");
    let content_string = fs::read_to_string(path);
    let content_string = match content_string {
        Ok(content) => content,
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };

    run(&content_string);
}

fn run_prompt() {

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
        run(&result.to_string());
    }
}

fn run(content: &String){
    println!("Tokens:");
    let tokens: Vec<&str> = content.split(" ").collect();
    for token in tokens {
        println!("\t- {token}");
    }
}
