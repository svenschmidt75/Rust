#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tokens {
    Identifier(String),
    Constant(i64),
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

impl Tokens {
    pub fn to_string(&self) -> String {
        match self {
            Tokens::Identifier(name) => format!("Identifier({})", name),
            Tokens::Constant(val) => format!("Constant({})", val),
            Tokens::Int => "Int".to_string(),
            Tokens::Void => "Void".to_string(),
            Tokens::Return => "Return".to_string(),
            Tokens::OpenParen => "(".to_string(),
            Tokens::CloseParen => ")".to_string(),
            Tokens::OpenBrace => "{".to_string(),
            Tokens::CloseBrace => "}".to_string(),
            Tokens::Semicolon => ";".to_string(),
            Tokens::EOF => "EOF".to_string(),
        }
    }
}
