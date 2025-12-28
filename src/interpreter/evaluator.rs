use crate::parser::Program;
use super::environment::Environment;
use super::runtime_value::RuntimeValue;
use super::error::RuntimeError;

pub struct WalkerEvaluator {
    // fields omitted
    code: Program,
    environment: Environment,
}

impl WalkerEvaluator {
    pub fn new() -> Self {
        todo!()
    }

    pub fn interpret(&mut self, program: Program) -> Result<RuntimeValue, RuntimeError> {
        todo!()
    }
}