#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tokens {
    Identifier(String),
    Constant(usize),
    Int,
    Void,
    Return,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    EOF,
}