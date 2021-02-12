use std::iter::Peekable;
use std::str::Chars;
use super::tokens::Token;

#[derive(Debug)]
pub struct TokenizeError {
    message: String
}

impl TokenizeError {
    pub fn new(message: &str) -> Self {
        Self {
            message: String::from(message)
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

pub fn tokenize(source: &String) -> TokenizeResult<Vec<Token>> {

    let mut chars = source.chars().peekable();
    skip_spaces(&mut chars);

    let mut tokens = Vec::new();

    while let Some(_) = chars.peek() {
        let token = next_token(&mut chars)?;
        tokens.push(token);

        skip_spaces(&mut chars);
    }

    Ok(tokens)

}

fn skip_spaces(chars: &mut Peekable<Chars>) {
    while chars.peek().map_or(false, |c| c.is_ascii_whitespace()) {
        chars.next();
    }
}

fn next_token(chars: &mut Peekable<Chars>) -> TokenizeResult<Token> {
    if let Some(c) = chars.peek() {
        match *c {
            'a'..='z' | 'A'..='Z' | ':' => {
                read_symbol_or_ident(chars)
            },
            '0'..='9' => {
                read_number(chars)
            },
            '"' => {
                read_string(chars)
            },
            '\\' => {
                read_char(chars)
            },
            _ => single_char_token(chars)
        }
    } else {
        Err(TokenizeError::new("Unexpected End of File."))
    }
}

fn read_symbol_or_ident(chars: &mut Peekable<Chars>) -> TokenizeResult<Token> {
    let mut value = String::new();

    while let Some(c) = chars.peek() {
        if c.is_ascii_whitespace() {
            break;
        } else {
            value.push(chars.next().unwrap());
        }
    }

    if value.starts_with(':') {
        Ok(Token::Ident(value))
    } else {
        Ok(Token::Symbol(value))
    }
}

fn read_number(chars: &mut Peekable<Chars>) -> TokenizeResult<Token> {
    let mut value: i64 = 0;

    // TODO: Different representations
    while let Some(c) = chars.peek() {
        if c.is_digit(10) {
            value = value * 10 + i64::from(chars.next().unwrap().to_digit(10).unwrap());
        } else {
            break;
        }
    }

    Ok(Token::Integer(value))
}

fn read_string(chars: &mut Peekable<Chars>) -> TokenizeResult<Token> {
    let mut value = String::new();

    assert_eq!('"', chars.next().unwrap());

    loop {
        match chars.peek() {
            Some('"') => {
                chars.next();
                break;
            },
            Some('\n') => return Err(TokenizeError::new("Unexpected end of line while parsing a string")),
            Some(_) => {
                value.push(chars.next().unwrap());
            },
            None => return Err(TokenizeError::new("Unexpected End of File while parsing String"))
        }
    }

    assert_eq!('"', chars.next().unwrap());

    Ok(Token::String(value))
}

fn read_char(chars: &mut Peekable<Chars>) -> TokenizeResult<Token> {
    assert_eq!('\\', chars.next().unwrap());
    if let Some(c) = chars.next() {
        Ok(Token::Char(c))
    } else {
        Err(TokenizeError::new("Unexpected End of File while parsing Char"))
    }
}

fn single_char_token(chars: &mut Peekable<Chars>) -> TokenizeResult<Token> {
    let token = match chars.peek() {
        Some('(') => Ok(Token::LParen),
        Some(')') => Ok(Token::RParen),
        Some('[') => Ok(Token::LBrack),
        Some(']') => Ok(Token::RBrack),
        Some('{') => Ok(Token::LCurl),
        Some('}') => Ok(Token::RCurl),
        Some('#') => Ok(Token::Hash),
        Some(',') => Ok(Token::Comma),
        Some('\'') => Ok(Token::SingleQuote),
        Some(c) => Err(TokenizeError::new(&format!("Unexpected Character: {}", c))),
        _ => Err(TokenizeError::new("Unexpected End of File"))
    };
    if token.is_ok() {
        chars.next();
    }
    token
}