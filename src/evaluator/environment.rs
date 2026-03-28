use super::value::TrackedValue;
use crate::common::Symbol;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Environment {
    pub parent: Option<Rc<RefCell<Environment>>>,
    pub values: HashMap<Symbol, TrackedValue>,
    pub types: HashMap<Symbol, crate::common::Type>,
}

impl Environment {
    pub fn new() -> Self {
        Environment { 
            parent: None, 
            values: HashMap::new(),
            types: HashMap::new(),
        }
    }

    pub fn extend(parent: Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(Self {
            parent: Some(parent),
            values: HashMap::new(),
            types: HashMap::new(),
        }))
    }

    pub fn get(&self, symbol: &Symbol) -> Option<TrackedValue> {
        if let Some(value) = self.values.get(symbol) {
            Some(value.clone())
        } else if let Some(enclosing) = &self.parent {
            enclosing.borrow().get(symbol)
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
        if let Some(enclosing) = &mut self.parent {
            if enclosing.borrow_mut().assign(symbol, value.clone()) {
                return true;
            }
        }
        
        false
    }

    pub fn define(&mut self, symbol: Symbol, value: TrackedValue) {
        let value_type = value.get_type();
        println!("Defining variable {:?} with type {:?}", symbol, value_type);
        self.types.insert(symbol, value.get_type());
        self.values.insert(symbol, value);
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