use crate::nodes::NodePtr;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct EvalContext {
    variables: HashMap<String, NodePtr>,
    parent: Option<Rc<EvalContext>>,
}

impl EvalContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            parent: None
        }
    }

    pub fn _new_from(parent: &Rc<Self>) -> Self {
        Self {
            variables: HashMap::new(),
            parent: Some(parent.clone())
        }
    }

    fn get_var(&self, name: &str) -> Option<&NodePtr> {
        self.variables.get(name)
    }

    fn set_var(&mut self, name: &str, value: NodePtr) {
        self.variables.insert(name.to_string(), value);
    }

    fn parent(&self) -> &Option<Rc<EvalContext>> {
        &self.parent
    }

    fn parent_mut(&mut self) -> &mut Option<Rc<EvalContext>> {
        &mut self.parent
    }
}
