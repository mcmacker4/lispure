#[derive(Debug)]
pub struct EvalError {
    message: String
}

impl EvalError {
    pub fn new(message: &str) -> Self {
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
