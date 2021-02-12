use std::rc::Rc;
use std::collections::{HashSet, HashMap};
use std::hash::{Hash, Hasher};

pub enum Node {
    Nil,
    List(NodePtr, NodePtr),
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

pub type NodePtr = Rc<Box<Node>>;

impl Node {
    pub fn print(&self) {
        match self {
            Node::Nil => print!("Nil"),
            Node::List(_, _) => self.print_list(),
            Node::Vector(_) => unimplemented!(),
            Node::Set(_) => unimplemented!(),
            Node::Map(_) => unimplemented!(),
            Node::Symbol(symbol) => print!("{}", symbol),
            Node::Ident(ident) => print!("{}", ident),
            Node::String(string) => print!("\"{}\"", string),
            Node::Char(char) => print!("\\{}", char),
            Node::Integer(int) => print!("{}", int),
            Node::Float(float) => print!("{}", float)
        }
    }

    fn print_list(&self) {
        print!("(");
        let mut node = self;
        while let Node::List(left, right) = node {
            left.print();
            if let Node::List(_, _) = ***right {
                print!(" ");
            }
            node = right;
        }
        print!(")")
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