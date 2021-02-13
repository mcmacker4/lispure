
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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TokenKind::LParen => write!(f, "'('"),
            TokenKind::RParen => write!(f, "')'"),
            TokenKind::LBrack => write!(f, "'['"),
            TokenKind::RBrack => write!(f, "']'"),
            TokenKind::LCurl => write!(f, "'{{'"),
            TokenKind::RCurl => write!(f, "'}}'"),
            TokenKind::SingleQuote => write!(f, "'\"'"),
            TokenKind::Hash => write!(f, "'#'"),
            TokenKind::Comma => write!(f, "','"),
            TokenKind::Symbol(s) => write!(f, "'{}'", s),
            TokenKind::Ident(i) => write!(f, "'{}'", i),
            TokenKind::String(s) => write!(f, "'{}'", s),
            TokenKind::Char(c) => write!(f, "'{}'", c),
            TokenKind::Integer(i) => write!(f, "'{}'", i),
            TokenKind::Float(n) => write!(f, "'{}'", n),
        }
    }
}