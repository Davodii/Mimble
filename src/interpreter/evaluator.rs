use crate::parser::Stmt;
use super::environment::Environment;
use super::runtime_value::RuntimeValue;
use super::error::RuntimeError;

pub struct WalkerEvaluator {
    // fields omitted
    code: Vec<Stmt>,
    environment: Environment,
}

impl WalkerEvaluator {
    pub fn new() -> Self {
        todo!()
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<RuntimeValue, RuntimeError> {
        todo!()
    }
}