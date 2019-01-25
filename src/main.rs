use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process;

mod errors;
mod scanner;
mod tokens;

use errors::LoxResult;
use scanner::Scanner;

fn run(script: &str) -> LoxResult {
    println!("Running: {}", script);

    let scanner = Scanner::new(script);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }

    Ok(())
}

fn run_file(path: &str) -> LoxResult {
    println!("Running file {}", path);

    let mut file = File::open(path)?;
    let mut script_content = String::new();
    file.read_to_string(&mut script_content)?;

    run(&script_content)
}

fn run_prompt() -> LoxResult {
    println!("Running prompt.");

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut buffer = String::new();
    loop {
        write!(stdout, "> ")?;
        stdout.flush()?;

        stdin.read_line(&mut buffer)?;

        if buffer.len() == 0 {
            return Ok(());
        }

        run(&buffer)?;
        buffer.clear();
    }
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    let result = match args.len() {
        0 => run_prompt(),
        1 => run_file(args[0].as_str()),
        _ => {
            eprintln!("Usage: lox [script]");
            process::exit(64);
        }
    };

    match result {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Error when running script: {}", err);
            process::exit(1);
        }
    }
}
