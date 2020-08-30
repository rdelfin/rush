use std::iter::Peekable;
use thiserror::Error;

use crate::parser::tokens::{ASTToken, LexerToken};

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("We were unable to identify the command name")]
    CommandNameNotFound,
}

pub fn parser_parse<'a, T: Iterator<Item = &'a LexerToken>>(
    iter: &mut T,
) -> Result<Vec<ASTToken>, ParserError> {
    let mut res = vec![];

    let mut iter = iter.peekable();

    loop {
        let t = match iter.peek() {
            None => {
                break;
            }
            Some(t) => t,
        };

        let new_tokens = match t {
            LexerToken::Text(_) => vec![parse_command(&mut iter)?],
            // Extra new-lines are ignored
            LexerToken::NewLine => {
                iter.next();
                vec![]
            }
        };

        res.extend(new_tokens);
    }

    Ok(res)
}

fn parse_command<'a, T: Iterator<Item = &'a LexerToken>>(
    iter: &mut Peekable<T>,
) -> Result<ASTToken, ParserError> {
    // Name must be present
    let name = match iter.next() {
        None => {
            return Err(ParserError::CommandNameNotFound);
        }
        Some(LexerToken::Text(s)) => s,
        Some(_) => {
            return Err(ParserError::CommandNameNotFound);
        }
    };

    let mut args = vec![];

    loop {
        let next_t = match iter.peek() {
            None => {
                break;
            }
            Some(t) => t,
        };

        match next_t {
            LexerToken::Text(s) => args.push(s.clone()),
            _ => {
                break;
            }
        }

        iter.next();
    }

    Ok(ASTToken::Command {
        name: name.clone(),
        args,
    })
}
