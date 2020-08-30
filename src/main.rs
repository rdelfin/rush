use anyhow::Result;
use std::io::{self, BufRead, Write};

use crate::parser::{lexer_tokenize, parser_parse};

mod parser;

fn main() -> Result<()> {
    loop {
        print_prompt()?;

        let next_line = read_line()?;

        let lexer_tokens = match lexer_tokenize(&next_line) {
            Err(e) => {
                eprintln!("There was an error parsing the line:\n\t{}", e);
                continue;
            }
            Ok(s) => s,
        };

        let tokens = match parser_parse(&mut lexer_tokens.iter()) {
            Err(e) => {
                eprintln!("There was an error parsing the line:\n\t{}", e);
                continue;
            }
            Ok(t) => t,
        };

        println!("TOKENS:");
        for token in tokens {
            println!("\t{:?}", token);
        }
    }
}

fn print_prompt() -> Result<()> {
    print!("$ ");
    io::stdout().flush()?;
    Ok(())
}

fn read_line() -> Result<String> {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line)?;
    Ok(line)
}
