mod lexer;
mod parser;
mod tokens;

pub use self::{lexer::LexingError, parser::ParserError, tokens::ASTToken};

use self::{lexer::lexer_tokenise, parser::parser_parse};
use anyhow::Result;

pub fn parse(code: &str) -> Result<Vec<ASTToken>> {
    let lexer_tokens = lexer_tokenise(code)?;
    Ok(parser_parse(lexer_tokens.iter())?)
}
