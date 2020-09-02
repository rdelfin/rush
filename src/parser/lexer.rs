use std::{iter::Peekable, str::Chars};
use thiserror::Error;

use crate::parser::tokens::LexerToken;

#[derive(Error, Debug)]
pub enum LexingError {
    #[error("We encountered a \"{0}\", which does not match any valid lexer tokens")]
    InvalidCharError(char),
    #[error("The text ended unexpectedly while parsing a {0}")]
    UnexpectedEnd(String),
}

pub fn lexer_tokenise(code: &str) -> Result<Vec<LexerToken>, LexingError> {
    let mut tokens = vec![];
    let mut char_iter = code.chars().peekable();
    // The order is important as it defines which matches are checked first
    let parsers: Vec<Box<dyn Token>> = vec![
        Box::new(TextParser),
        Box::new(NewLineParser),
        Box::new(WhitespaceParser),
    ];

    loop {
        match char_iter.peek() {
            Some(c) => {
                let c = *c;
                let mut found_match = false;

                for parser in parsers.iter() {
                    if parser.matches(c) {
                        found_match = true;
                        let new_tokens = parser.parse(&mut char_iter)?;
                        tokens.extend(new_tokens);
                        break;
                    }
                }

                if !found_match {
                    return Err(LexingError::InvalidCharError(c));
                };
            }
            None => break,
        }
    }

    Ok(tokens)
}

trait Token {
    fn matches(&self, c: char) -> bool;
    fn parse(&self, iter: &mut Peekable<Chars>) -> Result<Vec<LexerToken>, LexingError>;
}

struct TextParser;
struct NewLineParser;
struct WhitespaceParser;

impl Token for TextParser {
    fn matches(&self, c: char) -> bool {
        is_letter(c) || is_number(c) || is_valid_symbol(c)
    }

    fn parse(&self, iter: &mut Peekable<Chars>) -> Result<Vec<LexerToken>, LexingError> {
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

        Ok(vec![LexerToken::Text(str_buf)])
    }
}

impl Token for NewLineParser {
    fn matches(&self, c: char) -> bool {
        c == '\r' || c == '\n'
    }

    fn parse(&self, iter: &mut Peekable<Chars>) -> Result<Vec<LexerToken>, LexingError> {
        let c = iter.peek();

        match c {
            Some(&c) => {
                if c == '\n' {
                    iter.next();
                } else if c == '\r' {
                    if let Some(next_c) = iter.next() {
                        if next_c != '\n' {
                            return Err(LexingError::InvalidCharError(c));
                        }
                    } else {
                        return Err(LexingError::UnexpectedEnd("new line".to_string()));
                    }
                } else {
                    return Err(LexingError::InvalidCharError(c));
                }
            }
            None => {
                return Err(LexingError::UnexpectedEnd("new line".to_string()));
            }
        }

        return Ok(vec![LexerToken::NewLine]);
    }
}

impl Token for WhitespaceParser {
    fn matches(&self, c: char) -> bool {
        c.is_whitespace()
    }

    fn parse(&self, iter: &mut Peekable<Chars>) -> Result<Vec<LexerToken>, LexingError> {
        loop {
            if let Some(c) = iter.peek() {
                if c.is_whitespace() {
                    iter.next();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(vec![])
    }
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
