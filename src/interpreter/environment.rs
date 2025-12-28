use super::runtime_value::RuntimeValue;
use super::error::RuntimeError;

pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: std::collections::HashMap<String, RuntimeValue>,
}

impl Environment {
    pub fn new() -> Self {
        todo!()
    }

    pub fn get(&self, name: &str) -> Option<&RuntimeValue> {
        todo!()
    }

    pub fn assign(&mut self, name: &str, value: RuntimeValue) -> Result<(), RuntimeError> {
        todo!()
    }

    pub fn enter_scope(&mut self) {
        todo!()
    }

    pub fn exit_scope(&mut self) {
        todo!()
    }
}