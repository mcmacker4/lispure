use crate::nodes::{Node, NodePtr};
use super::tokens::Token;
use std::iter::Peekable;
use std::slice::Iter;
use std::rc::Rc;

#[derive(Debug)]
pub struct ParseError {
    message: String
}

impl ParseError {
    fn new(message: &str) -> Self {
        Self {
            message: String::from(message)
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
            Token::LParen => parse_list(tokens)?,
            Token::LBrack => parse_vector(tokens)?,
            Token::Hash => parse_set(tokens)?,
            Token::LCurl => parse_map(tokens)?,
            Token::Ident(ident) => {
                tokens.next().unwrap();
                ptr(Node::Ident(ident.clone()))
            },
            Token::Symbol(symbol) => {
                tokens.next().unwrap();
                ptr(Node::Symbol(symbol.clone()))
            },
            Token::String(str) => {
                tokens.next().unwrap();
                ptr(Node::String(str.clone()))
            },
            Token::Char(c) => {
                tokens.next().unwrap();
                ptr(Node::Char(*c))
            },
            Token::Integer(int) => {
                tokens.next().unwrap();
                ptr(Node::Integer(*int))
            },
            Token::Float(float) => {
                tokens.next().unwrap();
                ptr(Node::Float(*float))
            },
            _ => panic!("Lol What?")
        };
        Ok(node)
    } else {
        Err(ParseError::new("Unexpected End of Token List"))
    }

}

fn parse_list(tokens: &mut TokenIter) -> ParseResult<NodePtr> {
    // Consume parenthesis
    tokens.next().unwrap();
    Ok(parse_list_rest(tokens)?)
}

fn parse_list_rest(tokens: &mut TokenIter) -> ParseResult<NodePtr> {
    let node = match tokens.peek() {
        Some(Token::RParen) => {
            tokens.next().unwrap();
            Node::Nil
        },
        Some(_) => {
            let left = parse_expr(tokens)?;
            let right = parse_list_rest(tokens)?;
            Node::List(left, right)
        },
        None => return Err(ParseError::new("Unexpected End of Token List while parsing List"))
    };
    Ok(ptr(node))
}

fn parse_vector(_tokens: &mut TokenIter) -> ParseResult<NodePtr> {
    unimplemented!()
}

fn parse_set(_tokens: &mut TokenIter) -> ParseResult<NodePtr> {
    unimplemented!()
}

fn parse_map(_tokens: &mut TokenIter) -> ParseResult<NodePtr> {
    unimplemented!()
}
