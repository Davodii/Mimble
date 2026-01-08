use crate::{common::Type, parser::LiteralValue};

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    Integer(i64),
    Float(f64),
    // TODO: maybe differentiate between static strings and dynamic strings
    String(String),
    Boolean(bool),
    Nil,
    Array {
        elements: Vec<RuntimeValue>,
        element_type: Option<Type>,
    },
    // Other variants omitted
    // Function, 
}

impl From<LiteralValue> for RuntimeValue {
    fn from(lit: LiteralValue) -> Self {
        match lit {
            LiteralValue::Integer(n) => RuntimeValue::Integer(n),
            LiteralValue::Float(n) => RuntimeValue::Float(n),
            LiteralValue::String(s) => RuntimeValue::String(s),
            LiteralValue::Boolean(b) => RuntimeValue::Boolean(b),
            LiteralValue::Nil => RuntimeValue::Nil,
        }
    }
}

impl RuntimeValue {
    pub fn get_type(&self) -> Type {
        match self {
            RuntimeValue::Integer(_) => Type::Integer,
            RuntimeValue::Float(_) => Type::Float,
            RuntimeValue::String(_) => Type::String,
            RuntimeValue::Boolean(_) => Type::Boolean,
            RuntimeValue::Nil => Type::Nil,
            RuntimeValue::Array { elements, element_type } => {
                if let Some(t) = element_type {
                    Type::Array(Box::new(t.clone()))
                } else if let Some(first) = elements.first() {
                    Type::Array(Box::new(first.get_type()))
                } else {
                    // Default to Array of Nil for empty arrays
                    Type::Array(Box::new(Type::Nil))
                }
            },
        }
    }

    pub fn add(self, other: &RuntimeValue) -> Result<RuntimeValue, ()> {
        match (self, other) {
            (RuntimeValue::Integer(a), RuntimeValue::Integer(b)) => Ok(RuntimeValue::Integer(a + b)),
            (RuntimeValue::Float(a), RuntimeValue::Float(b)) => Ok(RuntimeValue::Float(a + b)),
            (RuntimeValue::String(a), RuntimeValue::String(b)) => Ok(RuntimeValue::String(a + &b)),
            _ => Err(()),
        }
    }

    pub fn subtract(self, other: &RuntimeValue) -> Result<RuntimeValue, ()> {
        match (self, other) {
            (RuntimeValue::Integer(a), RuntimeValue::Integer(b)) => Ok(RuntimeValue::Integer(a - b)),
            (RuntimeValue::Float(a), RuntimeValue::Float(b)) => Ok(RuntimeValue::Float(a - b)),
            _ => Err(()),
        }
    }

    pub fn multiply(self, other: &RuntimeValue) -> Result<RuntimeValue, ()> {
        match (self, other) {
            (RuntimeValue::Integer(a), RuntimeValue::Integer(b)) => Ok(RuntimeValue::Integer(a * b)),
            (RuntimeValue::Float(a), RuntimeValue::Float(b)) => Ok(RuntimeValue::Float(a * b)),
            _ => Err(()),
        }
    }

    pub fn divide(self, other: &RuntimeValue) -> Result<RuntimeValue, ()> {
        match (self, other) {
            (RuntimeValue::Integer(a), RuntimeValue::Integer(b)) => {
                if *b == 0 {
                    Err(())
                } else {
                    Ok(RuntimeValue::Integer(a / b))
                }
            },
            (RuntimeValue::Float(a), RuntimeValue::Float(b)) => {
                if *b == 0.0 {
                    Err(())
                } else {
                    Ok(RuntimeValue::Float(a / b))
                }
            },
            _ => Err(()),
        }
    }

    pub fn modulus(self, other: &RuntimeValue) -> Result<RuntimeValue, ()> {
        match (self, other) {
            (RuntimeValue::Integer(a), RuntimeValue::Integer(b)) => {
                if *b == 0 {
                    Err(())
                } else {
                    Ok(RuntimeValue::Integer(a % b))
                }
            },
            (RuntimeValue::Float(a), RuntimeValue::Float(b)) => {
                if *b == 0.0 {
                    Err(())
                } else {
                    Ok(RuntimeValue::Float(a % b))
                }
            },
            _ => Err(()),
        }
    }

    pub fn and(self, other: &RuntimeValue) -> Result<RuntimeValue, ()> {
        match (self, other) {
            (RuntimeValue::Boolean(a), RuntimeValue::Boolean(b)) => Ok(RuntimeValue::Boolean(a && *b)),
            _ => Err(()),
        }
    }

    pub fn or(self, other: &RuntimeValue) -> Result<RuntimeValue, ()> {
        match (self, other) {
            (RuntimeValue::Boolean(a), RuntimeValue::Boolean(b)) => Ok(RuntimeValue::Boolean(a || *b)),
            _ => Err(()),
        }
    }

    pub fn equals(&self, other: &RuntimeValue) -> Result<RuntimeValue, ()> {
        match (self, other) {
            (RuntimeValue::Integer(a), RuntimeValue::Integer(b)) => Ok(RuntimeValue::Boolean(a == b)),
            (RuntimeValue::Float(a), RuntimeValue::Float(b)) => Ok(RuntimeValue::Boolean(a == b)),
            (RuntimeValue::String(a), RuntimeValue::String(b)) => Ok(RuntimeValue::Boolean(a == b)),
            (RuntimeValue::Boolean(a), RuntimeValue::Boolean(b)) => Ok(RuntimeValue::Boolean(a == b)),
            _ => Err(()),
        }
    }

    pub fn not_equals(&self, other: &RuntimeValue) -> Result<RuntimeValue, ()> {
        match (self, other) {
            (RuntimeValue::Integer(a), RuntimeValue::Integer(b)) => Ok(RuntimeValue::Boolean(a != b)),
            (RuntimeValue::Float(a), RuntimeValue::Float(b)) => Ok(RuntimeValue::Boolean(a != b)),
            (RuntimeValue::String(a), RuntimeValue::String(b)) => Ok(RuntimeValue::Boolean(a != b)),
            (RuntimeValue::Boolean(a), RuntimeValue::Boolean(b)) => Ok(RuntimeValue::Boolean(a != b)),
            _ => Err(()),
        }
    }

    pub fn less_than(&self, other: &RuntimeValue) -> Result<RuntimeValue, ()> {
        match (self, other) {
            (RuntimeValue::Integer(a), RuntimeValue::Integer(b)) => Ok(RuntimeValue::Boolean(a < b)),
            (RuntimeValue::Float(a), RuntimeValue::Float(b)) => Ok(RuntimeValue::Boolean(a < b)),
            _ => Err(()),
        }
    }

    pub fn less_than_equal(&self, other: &RuntimeValue) -> Result<RuntimeValue, ()> {
        match (self, other) {
            (RuntimeValue::Integer(a), RuntimeValue::Integer(b)) => Ok(RuntimeValue::Boolean(a <= b)),
            (RuntimeValue::Float(a), RuntimeValue::Float(b)) => Ok(RuntimeValue::Boolean(a <= b)),
            _ => Err(()),
        }
    }

    pub fn greater_than(&self, other: &RuntimeValue) -> Result<RuntimeValue, ()> {
        match (self, other) {
            (RuntimeValue::Integer(a), RuntimeValue::Integer(b)) => Ok(RuntimeValue::Boolean(a > b)),
            (RuntimeValue::Float(a), RuntimeValue::Float(b)) => Ok(RuntimeValue::Boolean(a > b)),
            _ => Err(()),
        }
    }

    pub fn greater_than_equal(&self, other: &RuntimeValue) -> Result<RuntimeValue, ()> {
        match (self, other) {
            (RuntimeValue::Integer(a), RuntimeValue::Integer(b)) => Ok(RuntimeValue::Boolean(a >= b)),
            (RuntimeValue::Float(a), RuntimeValue::Float(b)) => Ok(RuntimeValue::Boolean(a >= b)),
            _ => Err(()),
        }
    }

    pub fn not(self) -> Result<RuntimeValue, ()> {
        match self {
            RuntimeValue::Boolean(a) => Ok(RuntimeValue::Boolean(!a)),
            _ => Err(()),
        }
    }

    pub fn negate(self) -> Result<RuntimeValue, ()> {
        match self {
            RuntimeValue::Integer(a) => Ok(RuntimeValue::Integer(-a)),
            RuntimeValue::Float(a) => Ok(RuntimeValue::Float(-a)),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeValue::Integer(n) => write!(f, "{}", n),
            RuntimeValue::Float(n) => write!(f, "{}", n),
            RuntimeValue::String(s) => write!(f, "{}", s),
            RuntimeValue::Boolean(b) => write!(f, "{}", b),
            RuntimeValue::Nil => write!(f, "nil"),
            RuntimeValue::Array{elements, ..} => {
                let elements: Vec<String> = elements.iter().map(|v| format!("{}", v)).collect();
                write!(f, "[{}]", elements.join(", "))
            },
        }
    }
}