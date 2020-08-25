use crate::parser::tokens::Token;

fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut char_iter = code.chars().peekable();
    loop {
        match char_iter.peek() {
            Some(c) => {}
            None => break,
        }
    }

    tokens
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
