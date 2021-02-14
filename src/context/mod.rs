use crate::nodes::NodePtr;
use std::collections::HashMap;

pub trait Context {
    fn get_var(&self, name: &str) -> Option<&NodePtr>;
    fn set_var(&mut self, name: &str, value: NodePtr);
}

pub struct EvalContext {
    variables: HashMap<String, NodePtr>
}

impl EvalContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new()
        }
    }
}

impl Context for EvalContext {
    fn get_var(&self, name: &str) -> Option<&NodePtr> {
        self.variables.get(name)
    }

    fn set_var(&mut self, name: &str, value: NodePtr) {
        self.variables.insert(name.to_string(), value);
    }
}