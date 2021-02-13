
#[derive(Debug, Copy, Clone)]
pub struct TokenPos {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub enum TokenKind {
    LParen,
    RParen,
    LBrack,
    RBrack,
    LCurl,
    RCurl,
    SingleQuote,
    Hash,
    Comma,
    Symbol(String),
    Ident(String),
    String(String),
    Char(char),
    Integer(i64),
    Float(f64),
}

#[derive(Debug)]
pub struct Token(pub TokenKind, pub TokenPos);