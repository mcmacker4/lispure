
#[derive(Debug)]
pub enum Token {
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
