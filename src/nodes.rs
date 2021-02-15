use std::rc::Rc;
use std::collections::{HashSet, HashMap};
use std::hash::{Hash, Hasher};
use std::iter::IntoIterator;

#[derive(Debug)]
pub enum Node {
    Nil,
    List(NodePtr, NodePtr, bool),
    Vector(Vec<NodePtr>),
    Set(HashSet<NodePtr>),
    Map(HashMap<NodePtr, NodePtr>),
    Symbol(String),
    Ident(String),
    String(String),
    Char(char),
    Integer(i64),
    Float(f64)
}

pub type NodePtr = Rc<Node>;

impl Node {

    pub fn is_list(&self) -> bool {
        match self {
            Self::List(_, _, _) => true,
            _ => false
        }
    }

    pub fn len(&self) -> Option<usize> {
        match self {
            Self::List(_, right, _) => {
                if let Self::Nil = right.as_ref() {
                    Some(1)
                } else{
                    Some(1 + right.len().expect("List's right node is neither list or nil"))
                }
            },
            Self::Vector(vec) => Some(vec.len()),
            Self::Set(set) => Some(set.len()),
            _ => None
        }
    }

}

pub trait IntoListIter {
    fn list_iter(&self) -> NodeIter;
}

impl IntoListIter for NodePtr {

    fn list_iter(&self) -> NodeIter {
        NodeIter {
            node: self.clone()
        }
    }

}

pub struct NodeIter {
    node: NodePtr
}

impl Iterator for NodeIter {
    type Item = NodePtr;

    fn next(&mut self) -> Option<Self::Item> {
        match self.node.clone().as_ref() {
            Node::List(left, right, _) => {
                self.node = right.clone();
                Some(left.clone())
            },
            Node::Nil => None,
            _ => panic!("Iterating over non-list node.")
        }
    }
}


impl std::fmt::Display for Node {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Nil => write!(f, "Nil"),
            Node::List(_, _, _) => {
                write!(f, "(")?;
                let mut node = self;
                while let Node::List(left, right, _) = node {
                    write!(f, "{}", left)?;
                    if let Node::List(_, _, _) = **right {
                        write!(f, " ")?;
                    }
                    node = right;
                }
                write!(f, ")")
            },
            Node::Vector(vec) => {
                write!(f, "[")?;
                let mut iter = vec.iter().peekable();
                while let Some(node) = iter.next() {
                    write!(f, "{}", node)?;
                    if let Some(_) = iter.peek() {
                        write!(f, " ")?;
                    }
                }
                write!(f, "]")
            },
            Node::Set(_) => unimplemented!(),
            Node::Map(_) => unimplemented!(),
            Node::Symbol(symbol) => write!(f, "{}", symbol),
            Node::Ident(ident) => write!(f, "{}", ident),
            Node::String(string) => write!(f, "\"{}\"", string),
            Node::Char(char) => write!(f, "\\{}", char),
            Node::Integer(int) => write!(f, "{}", int),
            Node::Float(float) => write!(f, "{}", float)
        }
    }

}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => true,
            (Self::Symbol(a), Self::Symbol(b)) => PartialEq::eq(a, b),
            (Self::Ident(a), Self::Ident(b)) => PartialEq::eq(a, b),
            (Self::Integer(a), Self::Integer(b)) => PartialEq::eq(a, b),
            _ => false
        }
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Symbol(symbol) => symbol.hash(state),
            Self::Ident(ident) => ident.hash(state),
            Self::String(string) => string.hash(state),
            Self::Char(char) => char.hash(state),
            Self::Integer(int) => int.hash(state),
            _ => panic!("Trying to hash an unhashable node.")
        }
    }
}