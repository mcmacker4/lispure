use std::iter::Peekable;
use std::str::Chars;
use super::tokens::{Token, TokenPos};
use crate::parser::tokens::TokenKind;

#[derive(Debug)]
pub struct TokenizeError {
    message: String,
    pos: TokenPos,
}

impl TokenizeError {
    pub fn new(message: &str, pos: TokenPos) -> Self {
        Self {
            message: String::from(message),
            pos
        }
    }
}

impl std::fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TokenizeError {}

pub type TokenizeResult<T> = std::result::Result<T, TokenizeError>;

struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,

    pub line: usize,
    pub column: usize,
}


impl<'a> Tokenizer<'a> {
    fn new(chars: Peekable<Chars<'a>>) -> Self {
        Self {
            chars,
            line: 1,
            column: 1
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn next(&mut self) -> Option<char> {
        if self.peek().map_or(false, |c| *c == '\n') {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        self.chars.next()
    }

    fn pos(&self) -> TokenPos {
        TokenPos {
            line: self.line,
            column: self.column
        }
    }
}

pub fn tokenize(source: &str) -> TokenizeResult<Vec<Token>> {

    let mut tokenizer = Tokenizer::new(source.chars().peekable());
    skip_spaces(&mut tokenizer);

    let mut tokens = Vec::new();

    while let Some(_) = tokenizer.peek() {
        let token = next_token(&mut tokenizer)?;
        tokens.push(token);

        skip_spaces(&mut tokenizer);
    }

    Ok(tokens)

}

fn skip_spaces(tokenizer: &mut Tokenizer) {
    while tokenizer.peek().map_or(false, |c| c.is_ascii_whitespace()) {
        tokenizer.next();
    }
}

fn next_token(tokenizer: &mut Tokenizer) -> TokenizeResult<Token> {
    if let Some(c) = tokenizer.peek() {
        match *c {
            'a'..='z' | 'A'..='Z' | ':' => {
                read_symbol_or_ident(tokenizer)
            },
            '0'..='9' => {
                read_number(tokenizer)
            },
            '"' => {
                read_string(tokenizer)
            },
            '\\' => {
                read_char(tokenizer)
            },
            _ => single_char_token(tokenizer)
        }
    } else {
        Err(TokenizeError::new("Unexpected End of File.", tokenizer.pos()))
    }
}

#[inline]
fn is_reserved_char(c: char) -> bool {
    match c {
        '[' | ']' | '(' | ')' | '{' | '}' | '\'' | '"' => true,
        _ => false
    }
}

fn read_symbol_or_ident(tokenizer: &mut Tokenizer) -> TokenizeResult<Token> {
    let mut value = String::new();

    let pos = tokenizer.pos();

    while let Some(c) = tokenizer.peek() {
        if c.is_ascii_whitespace() || is_reserved_char(*c) {
            break;
        } else {
            value.push(tokenizer.next().unwrap());
        }
    }

    if value.starts_with(':') {
        Ok(Token(TokenKind::Ident(value), pos))
    } else {
        Ok(Token(TokenKind::Symbol(value), pos))
    }
}

fn read_number(tokenizer: &mut Tokenizer) -> TokenizeResult<Token> {
    let mut value: i64 = 0;
    let pos = tokenizer.pos();

    // TODO: Different representations
    while let Some(c) = tokenizer.peek() {
        if c.is_digit(10) {
            value = value * 10 + i64::from(tokenizer.next().unwrap().to_digit(10).unwrap());
        } else {
            break;
        }
    }

    Ok(Token(TokenKind::Integer(value), pos))
}

fn read_string(tokenizer: &mut Tokenizer) -> TokenizeResult<Token> {
    let mut value = String::new();
    let pos = tokenizer.pos();

    assert_eq!('"', tokenizer.next().unwrap());

    loop {
        match tokenizer.peek() {
            Some('"') => {
                tokenizer.next();
                break;
            },
            Some('\n') => return Err(TokenizeError::new("Unexpected end of line while parsing a string", pos)),
            Some(_) => {
                value.push(tokenizer.next().unwrap());
            },
            None => return Err(TokenizeError::new("Unexpected End of File while parsing String", pos))
        }
    }

    Ok(Token(TokenKind::String(value), pos))
}

fn read_char(tokenizer: &mut Tokenizer) -> TokenizeResult<Token> {
    assert_eq!('\\', tokenizer.next().unwrap());
    let pos = tokenizer.pos();
    if let Some(c) = tokenizer.next() {
        Ok(Token(TokenKind::Char(c), pos))
    } else {
        Err(TokenizeError::new("Unexpected End of File while parsing Char", pos))
    }
}

fn single_char_token(tokenizer: &mut Tokenizer) -> TokenizeResult<Token> {
    let pos = tokenizer.pos();
    let token = match tokenizer.peek() {
        Some('(') => Ok(Token(TokenKind::LParen, pos)),
        Some(')') => Ok(Token(TokenKind::RParen, pos)),
        Some('[') => Ok(Token(TokenKind::LBrack, pos)),
        Some(']') => Ok(Token(TokenKind::RBrack, pos)),
        Some('{') => Ok(Token(TokenKind::LCurl, pos)),
        Some('}') => Ok(Token(TokenKind::RCurl, pos)),
        Some('#') => Ok(Token(TokenKind::Hash, pos)),
        Some(',') => Ok(Token(TokenKind::Comma, pos)),
        Some('\'') => Ok(Token(TokenKind::SingleQuote, pos)),
        Some(c) => Err(TokenizeError::new(&format!("Unexpected Character: {}", c), pos)),
        _ => Err(TokenizeError::new("Unexpected End of File", pos))
    };
    if token.is_ok() {
        tokenizer.next();
    }
    token
}