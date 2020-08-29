use std::iter::Peekable;
use thiserror::Error;

use crate::parser::tokens::LexerToken;

#[derive(Error, Debug)]
pub enum LexingError {
    #[error("We encountered a \"{0}\", which does not match any valid lexer tokens")]
    InvalidCharError(char),
}

pub fn lexer_tokenize(code: &str) -> Result<Vec<LexerToken>, LexingError> {
    let mut tokens = vec![];
    let mut char_iter = code.chars().peekable();
    loop {
        match char_iter.peek() {
            Some(c) => {
                let next_token = if is_letter(*c) || is_number(*c) || is_valid_symbol(*c) {
                    parse_text(&mut char_iter)
                } else if c.is_whitespace() {
                    parse_whitespace(&mut char_iter);
                    continue;
                } else {
                    return Err(LexingError::InvalidCharError(*c));
                };

                tokens.push(next_token);
            }
            None => break,
        }
    }

    Ok(tokens)
}

fn is_letter(c: char) -> bool {
    (c >= 'A' && c < 'Z') || (c >= 'a' && c <= 'z')
}

fn is_number(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_valid_symbol(c: char) -> bool {
    ['-', '_', '.', '/'].contains(&c)
}

fn parse_text<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> LexerToken {
    let mut str_buf = String::new();

    loop {
        if let Some(c) = iter.peek() {
            if is_letter(*c) || is_number(*c) || is_valid_symbol(*c) {
                str_buf.push(*c);
                iter.next();
            } else {
                break;
            }
        } else {
            break;
        }
    }

    LexerToken::Text(str_buf)
}

fn parse_whitespace<T: Iterator<Item = char>>(iter: &mut Peekable<T>) {
    loop {
        if let Some(c) = iter.peek() {
            if c.is_whitespace() {
                iter.next();
            } else {
                return;
            }
        } else {
            return;
        }
    }
}
