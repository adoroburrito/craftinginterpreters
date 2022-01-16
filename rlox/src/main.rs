use std::env;

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
}

fn run_prompt() {
    println!("Running prompt...");
}