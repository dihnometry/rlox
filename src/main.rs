use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

mod error;
mod scanner;
mod token;
mod token_type;

use scanner::Scanner;
use token::Token;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => run_file(&args[1]).expect("Could not run file."),
        n if n > 2 => {
            eprintln!("Usage: rlox [script]");
            process::exit(64);
        }
        _ => run_prompt(),
    }
}

fn run_file(path: &String) -> io::Result<()> {
    let content = fs::read_to_string(path)?;
    run(content);
    Ok(())
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Could not flush");
        let mut buff = String::new();
        match io::stdin().read_line(&mut buff) {
            Ok(_) => {
                if buff.is_empty() {
                    break;
                };
                run(buff);
            }
            Err(_) => println!("There was an error, try again."),
        };
    }
}

fn run(source: String) {
    let scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens();

    for token in tokens {
        println!("{token}");
    }
}
