use crate::nodes::{Node, NodePtr};
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
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}

type ParseResult<T> = std::result::Result<T, ParseError>;

type TokenIter<'a> = Peekable<Iter<'a, Token>>;

#[inline]
fn ptr(node: Node) -> NodePtr {
    Rc::new(Box::new(node))
}

pub fn parse_file(tokens: &mut TokenIter) -> ParseResult<NodePtr> {
    Ok(parse_file_rest(tokens)?)
}

fn parse_file_rest(tokens: &mut TokenIter) -> ParseResult<NodePtr> {
    let node = match tokens.peek() {
        Some(_) => {
            let left = parse_expr(tokens)?;
            let right = parse_file_rest(tokens)?;
            Node::List(left, right)
        },
        None => Node::Nil
    };
    Ok(ptr(node))
}

pub fn parse_expr(tokens: &mut TokenIter) -> ParseResult<NodePtr> {

    if let Some(token) = tokens.peek() {
        let node = match token {
            Token(TokenKind::LParen, _) => parse_list(tokens)?,
            Token(TokenKind::LBrack, _) => parse_vector(tokens)?,
            Token(TokenKind::Hash, _) => parse_set(tokens)?,
            Token(TokenKind::LCurl, _) => parse_map(tokens)?,
            Token(TokenKind::Ident(ident), _) => {
                tokens.next().unwrap();
                ptr(Node::Ident(ident.clone()))
            },
            Token(TokenKind::Symbol(symbol), _) => {
                tokens.next().unwrap();
                ptr(Node::Symbol(symbol.clone()))
            },
            Token(TokenKind::String(str), _) => {
                tokens.next().unwrap();
                ptr(Node::String(str.clone()))
            },
            Token(TokenKind::Char(c), _) => {
                tokens.next().unwrap();
                ptr(Node::Char(*c))
            },
            Token(TokenKind::Integer(int), _) => {
                tokens.next().unwrap();
                ptr(Node::Integer(*int))
            },
            Token(TokenKind::Float(float), _) => {
                tokens.next().unwrap();
                ptr(Node::Float(*float))
            },
            _ => panic!("Lol What?")
        };
        Ok(node)
    } else {
        Err(ParseError::new("Unexpected End of Token List", None))
    }

}

fn parse_list(tokens: &mut TokenIter) -> ParseResult<NodePtr> {
    // Consume parenthesis
    tokens.next().unwrap();
    Ok(parse_list_rest(tokens)?)
}

fn parse_list_rest(tokens: &mut TokenIter) -> ParseResult<NodePtr> {
    let node = match tokens.peek() {
        Some(Token(TokenKind::RParen, _)) => {
            tokens.next().unwrap();
            Node::Nil
        },
        Some(_) => {
            let left = parse_expr(tokens)?;
            let right = parse_list_rest(tokens)?;
            Node::List(left, right)
        },
        None => return Err(ParseError::new("Unexpected End of Token List while parsing List", None))
    };
    Ok(ptr(node))
}

fn parse_vector(tokens: &mut TokenIter) -> ParseResult<NodePtr> {
    let pos =  &tokens.next().unwrap().1;
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

fn parse_set(_tokens: &mut TokenIter) -> ParseResult<NodePtr> {
    unimplemented!()
}

fn parse_map(_tokens: &mut TokenIter) -> ParseResult<NodePtr> {
    unimplemented!()
}
