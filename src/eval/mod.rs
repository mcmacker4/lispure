mod error;

pub use error::EvalError;
use super::nodes::NodePtr;
use super::context::Context;
use crate::nodes::Node;
use std::rc::Rc;

pub type EvalResult = std::result::Result<NodePtr, EvalError>;


pub fn eval_file(context: &mut dyn Context, mut node: &NodePtr) -> EvalResult {

    let mut result: NodePtr = Rc::new(Node::Nil);

    while let Node::List(left, right, _) = node.as_ref() {
        result = eval_expr(context, left)?;
        node = right;
    }

    Ok(result)

}

fn eval_expr(context: &mut dyn Context, node: &NodePtr) -> EvalResult {
    match node.as_ref() {
        Node::List(_, _, literal) if !*literal => {
            call(context, node)
        }
        _ => Ok(node.clone())
    }
}

fn call(_context: &mut dyn Context, _node: &NodePtr) -> EvalResult {
    Ok(Rc::new(Node::Nil))
}