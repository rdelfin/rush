use anyhow::Result;
use std::io::{self, BufRead, Write};

mod executor;
mod parser;

fn main() -> Result<()> {
    loop {
        print_prompt()?;

        let next_line = read_line()?;

        let tokens = match parser::parse(&next_line) {
            Err(e) => {
                eprintln!("There was an error parsing the line:\n\t{}", e);
                continue;
            }
            Ok(t) => t,
        };

        executor::execute(tokens.iter())?;
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
