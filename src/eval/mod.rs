mod error;

pub use error::EvalError;
use super::nodes::NodePtr;
use crate::nodes::Node;
use std::rc::Rc;
use crate::context::EvalContext;

pub type EvalResult = std::result::Result<NodePtr, EvalError>;

pub fn eval_file(context: &mut EvalContext, mut node: &NodePtr) -> EvalResult {

    let mut result: NodePtr = Rc::new(Node::Nil);

    while let Node::List(left, right, _) = node.as_ref() {
        result = eval_expr(context, left)?;
        node = right;
    }

    Ok(result)

}

fn eval_expr(context: &mut EvalContext, node: &NodePtr) -> EvalResult {
    match node.as_ref() {
        Node::List(left, right, literal) if !*literal => {
            call(context, left, right)
        }
        _ => Ok(node.clone())
    }
}

fn call(context: &mut EvalContext, _left: &NodePtr, right: &NodePtr) -> EvalResult {
    match _left.as_ref() {
        Node::List(_, _, _) => unimplemented!(),
        Node::Symbol(symbol) => {
            if let Some(function) = context.root().get_builtin(symbol) {
                function(context, right)
            } else {
                Err(EvalError::new(&format!("Function '{}' not found", symbol)))
            }
        },
        node => Err(EvalError::new(&format!("Node {:?} is not a function", node)))
    }
}