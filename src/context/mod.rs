mod builtins;

use crate::nodes::NodePtr;
use std::collections::HashMap;
use std::rc::Rc;

use self::builtins::Builtin;
use crate::context::builtins::populate_builtins;

#[derive(Debug)]
pub struct EvalContext {
    variables: HashMap<String, NodePtr>,
    parent: Option<Rc<EvalContext>>,
    root: Rc<RootContext>
}

pub struct RootContext {
    builtins: HashMap<String, Builtin>,
}

impl std::fmt::Debug for RootContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Builtins: [")?;
        let mut builtins = self.builtins.keys().peekable();
        while let Some(key) = builtins.next() {
            write!(f, "{}", key)?;
            if builtins.peek().is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

impl EvalContext {
    pub fn new_main() -> Self {
        Self {
            variables: HashMap::new(),
            parent: None,
            root: Rc::new(RootContext::new())
        }
    }

    pub fn new_child(parent: &Rc<Self>) -> Self {
        Self {
            variables: HashMap::new(),
            parent: Some(parent.clone()),
            root: parent.root.clone()
        }
    }

    pub fn get_var(&self, name: &str) -> Option<&NodePtr> {
        self.variables.get(name)
    }

    pub fn set_var(&mut self, name: &str, value: NodePtr) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn parent(&self) -> &Option<Rc<EvalContext>> {
        &self.parent
    }

    pub fn parent_mut(&mut self) -> &mut Option<Rc<EvalContext>> {
        &mut self.parent
    }

    pub fn root(&self) -> &RootContext {
        &self.root
    }
}

impl RootContext {

    fn new() -> Self {
        let mut builtins = HashMap::new();
        populate_builtins(&mut builtins);
        Self {
            builtins
        }
    }

    pub fn get_builtin(&self, name: &str) -> Option<&Builtin> {
        self.builtins.get(name)
    }

    pub fn insert_builtin(&mut self, name: String, builtin: Builtin) {
        self.builtins.insert(name, builtin);
    }

}
