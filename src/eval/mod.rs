use crate::nodes::{NodeRef, Node};
use std::rc::Rc;

#[derive(Debug)]
pub struct EvalError {
    message: String
}

impl EvalError {
    fn new(message: &str) -> Self {
        Self {
            message: String::from(message)
        }
    }
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Eval Error: {}", self.message)
    }
}

impl std::error::Error for EvalError {}

type EvalResult = std::result::Result<NodeRef, EvalError>;


pub fn eval_file(mut node: &NodeRef) -> EvalResult {
    let mut last_result: NodeRef = Rc::new(Node::Nil);
    while let Node::List(left, right, _) = node.as_ref() {
        last_result = eval_expr(left)?;
        node = right;
    }
    Ok(last_result)
}

pub fn eval_expr(node: &NodeRef) -> EvalResult {
    Ok(node.clone())
}
