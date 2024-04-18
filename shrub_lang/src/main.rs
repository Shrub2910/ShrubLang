mod lexer;
mod token;

use std::env;
use std::fs;
use std::io;
use std::io::Write;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() > 2 {
        println!("Usage: shrub_lang [file_name]");
        return;
    } else if arguments.len() == 2 {
        println!("{}", &arguments[0]);
        run_file(&arguments[0]);
    } else {
        run_repl();
    }

    return;
}

fn run_file(file_name: &String) {
    let contents: String = fs::read_to_string(file_name).expect("Failed to read file");
    run(&contents);
}

fn run_repl() {
    loop {
        let mut input: String = String::new();
        print!(">>> ");
        io::stdout().flush().expect("Failed to flush buffer");
        match io::stdin().read_line(&mut input) {
            Ok(n) => {}
            Err(_) => {
                println!("Failed to read input!");
            }
        }

        println!("----------------");

        run(&input);
    }
}

fn run(contents: &String) {
    let tokens: Vec<token::Token> = lexer::scan_tokens(&contents);
    for new_token in tokens.iter() {
        println!("{}", new_token.to_string());
    }
}
