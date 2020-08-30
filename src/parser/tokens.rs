#[derive(Debug)]
pub enum LexerToken {
    Text(String),
    NewLine,
}

#[derive(Debug)]
pub enum ASTToken {
    Command { name: String, args: Vec<String> },
}
