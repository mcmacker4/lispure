use crate::nodes::{Node, NodeRef};
use super::tokens::Token;
use std::iter::Peekable;
use std::slice::Iter;
use std::rc::Rc;
use crate::parser::tokens::TokenKind;
use crate::parser::TokenPos;

#[derive(Debug)]
pub struct ParseError {
    message: String,
    pos: Option<TokenPos>
}

impl ParseError {
    fn new(message: &str, pos: Option<TokenPos>) -> Self {
        Self {
            message: String::from(message),
            pos
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(pos) = self.pos {
            write!(f, "Parse Error ({}:{}) {}", pos.line, pos.column, self.message)
        } else {
            write!(f, "Parse Error: {}", self.message)
        }
    }
}

impl std::error::Error for ParseError {}

type ParseResult<T> = std::result::Result<T, ParseError>;

type TokenIter<'a> = Peekable<Iter<'a, Token>>;

#[inline]
fn ptr(node: Node) -> NodeRef {
    Rc::new(node)
}

pub fn parse_file(tokens: &mut TokenIter) -> ParseResult<NodeRef> {
    Ok(parse_file_rest(tokens)?)
}

fn parse_file_rest(tokens: &mut TokenIter) -> ParseResult<NodeRef> {
    let node = match tokens.peek() {
        Some(_) => {
            let left = parse_expr(tokens)?;
            let right = parse_file_rest(tokens)?;
            Node::List(left, right, false)
        },
        None => Node::Nil
    };
    Ok(ptr(node))
}

pub fn parse_expr(tokens: &mut TokenIter) -> ParseResult<NodeRef> {

    if let Some(token) = tokens.next() {
        let node = match token {
            Token(TokenKind::LParen, pos) => {
                parse_list(tokens, pos, false)?
            },
            Token(TokenKind::LBrack, pos) => {
                parse_vector(tokens, pos)?
            },
            Token(TokenKind::Hash, _) => {
                parse_set(tokens)?
            },
            Token(TokenKind::LCurl, _) => {
                parse_map(tokens)?
            },
            Token(TokenKind::SingleQuote, _) => {
                match tokens.next() {
                    Some(Token(TokenKind::LParen, pos)) => {
                        parse_list(tokens, pos, true)?
                    },
                    Some(Token(kind, pos)) => {
                        return Err(ParseError::new(
                            &format!("Expected a list but got {}", kind),
                            Some(pos.clone())
                        ))
                    },
                    None => {
                        return Err(ParseError::new(
                            "Expected a list but got nothing.",
                            None
                        ))
                    }
                }
            }
            Token(TokenKind::Ident(ident), _) => {
                ptr(Node::Ident(ident.clone()))
            },
            Token(TokenKind::Symbol(symbol), _) => {
                ptr(Node::Symbol(symbol.clone()))
            },
            Token(TokenKind::String(str), _) => {
                ptr(Node::String(str.clone()))
            },
            Token(TokenKind::Char(c), _) => {
                ptr(Node::Char(*c))
            },
            Token(TokenKind::Integer(int), _) => {
                ptr(Node::Integer(*int))
            },
            Token(TokenKind::Float(float), _) => {
                ptr(Node::Float(*float))
            },
            Token(token, pos) => {
                return Err(ParseError::new(
                    &format!("Unexpected token {}", token),
                    Some(pos.clone())
                ))
            }
        };
        Ok(node)
    } else {
        Err(ParseError::new("Unexpected End of Token List", None))
    }

}

fn parse_list(tokens: &mut TokenIter, pos: &TokenPos, literal: bool) -> ParseResult<NodeRef> {
    let node = match tokens.peek() {
        Some(Token(TokenKind::RParen, _)) => {
            tokens.next().unwrap();
            Node::Nil
        },
        Some(_) => {
            let left = parse_expr(tokens)?;
            let right = parse_list(tokens, pos, false)?;
            Node::List(left, right, literal)
        },
        None => return Err(ParseError::new("Unexpected End of Token List while parsing List", Some(pos.clone())))
    };
    Ok(ptr(node))
}

fn parse_vector(tokens: &mut TokenIter, pos: &TokenPos) -> ParseResult<NodeRef> {
    let mut vector = Vec::new();
    loop {
        match tokens.peek() {
            Some(Token(TokenKind::RBrack, _)) => {
                tokens.next().unwrap();
                break;
            },
            Some(_) => {
                let node = parse_expr(tokens)?;
                vector.push(node);
            },
            None => {
                return Err(ParseError::new("Unexpected End of Token List while parsing Vector", Some(pos.clone())))
            }
        }
    }
    Ok(ptr(Node::Vector(vector)))
}

fn parse_set(_tokens: &mut TokenIter) -> ParseResult<NodeRef> {
    unimplemented!()
}

fn parse_map(_tokens: &mut TokenIter) -> ParseResult<NodeRef> {
    unimplemented!()
}
