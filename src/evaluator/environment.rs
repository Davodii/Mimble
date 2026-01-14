use super::value::TrackedValue;
use crate::common::Symbol;

pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: std::collections::HashMap<Symbol, TrackedValue>,
    types: std::collections::HashMap<Symbol, crate::common::Type>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { 
            enclosing: None, 
            values: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        }
    }

    pub fn new_with_enclosing(enclosing: Environment) -> Self {
        Environment { 
            enclosing: Some(Box::new(enclosing)), 
            values: std::collections::HashMap::new(),
            types: std::collections::HashMap::new(),
        }
    }

    pub fn get(&self, symbol: &Symbol) -> Option<TrackedValue> {
        if let Some(value) = self.values.get(symbol) {
            Some(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.get(symbol)
        } else {
            None
        }
    }

    pub fn assign(&mut self, symbol: &Symbol, value: TrackedValue) -> bool {
        if self.values.contains_key(symbol) {
            // Check if the types are the same
            if self.types.get(symbol) != Some(&value.get_type()) {
                // TODO: this is an error
                return false;
            }

            self.values.insert(symbol.clone(), value);
            return true;
        }

        // Check if the variable exists in the enclosing environments
        if let Some(enclosing) = &mut self.enclosing {
            if enclosing.assign(symbol, value.clone()) {
                return true;
            }
        }
        
        false
    }

    pub fn define(&mut self, symbol: Symbol, value: TrackedValue) {
        // TODO: unecessary cloning!
        self.values.insert(symbol.clone(), value.clone());
        self.types.insert(symbol.clone(), value.get_type().clone());
    }
}


impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Environment {{ ")?;
        for (key, value) in &self.values {
            write!(f, "{:?}: {:?}, ", key, value)?;
        }
        write!(f, "}}")
    }
}