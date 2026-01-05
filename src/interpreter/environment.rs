use super::runtime_value::RuntimeValue;
use crate::common::Symbol;
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: std::collections::HashMap<Symbol, RuntimeValue>,
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

    pub fn get(&self, symbol: &Symbol) -> Option<RuntimeValue> {
        if let Some(value) = self.values.get(symbol) {
            Some(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.get(symbol)
        } else {
            None
        }
    }

    pub fn assign(&mut self, symbol: &Symbol, value: RuntimeValue) -> bool {
        // Check if the variable exists in the enclosing environments
        if let Some(enclosing) = &mut self.enclosing {
            if enclosing.assign(symbol, value.clone()) {
                return true;
            }
        }
        if self.values.contains_key(symbol) {
            self.values.insert(symbol.clone(), value);
            true
        } else {
            false
        }
    }
}