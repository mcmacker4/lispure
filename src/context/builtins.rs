use crate::nodes::{NodePtr, Node, IntoListIter};
use std::collections::HashMap;
use crate::eval::{EvalResult, EvalError};
use crate::context::EvalContext;
use std::rc::Rc;
use std::convert::TryFrom;

pub type Builtin = fn(&mut EvalContext, &NodePtr) -> EvalResult;

fn add(_: &mut EvalContext, args: &NodePtr) -> EvalResult {
    if args.len().unwrap() == 2 {
        let mut iter = args.list_iter();
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        match (first.as_ref(), second.as_ref()) {
            (Node::Integer(a), Node::Integer(b)) => {
                Ok(Rc::new(Node::Integer(a + b)))
            },
            _ => Err(EvalError::new("add: invalid argument types"))
        }
    } else {
        Err(EvalError::new(&format!("Expected 2 arguments but got {}", args.len().unwrap())))
    }
}

pub fn populate_builtins(builtins: &mut HashMap<String, Builtin>) {
    builtins.insert("add".to_string(), add);
}

