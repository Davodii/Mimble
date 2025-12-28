use std::str::EncodeUtf16;

use super::runtime_value::RuntimeValue;
use super::error::RuntimeError;

pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: std::collections::HashMap<String, RuntimeValue>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { 
            enclosing: None, 
            values: std::collections::HashMap::new() 
        }
    }

    pub fn new_with_enclosing(enclosing: Environment) -> Self {
        Environment { 
            enclosing: Some(Box::new(enclosing)), 
            values: std::collections::HashMap::new() 
        }
    }

    pub fn get(&self, name: &str) -> Option<RuntimeValue> {
        if let Some(value) = self.values.get(name) {
            Some(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.get(name)
        } else {
            None
        }
    }

    pub fn assign(&mut self, name: &str, value: RuntimeValue) -> bool {
        // Check if the variable exists in the enclosing environments
        if let Some(enclosing) = &mut self.enclosing {
            if enclosing.assign(name, value.clone()) {
                return true;
            }
        }
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            true
        } else {
            false
        }
    }
}