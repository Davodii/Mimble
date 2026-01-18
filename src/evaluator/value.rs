use crate::{common::{Symbol, Type}, parser::LiteralValue};

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum DataSource {
    /// A standalone variable (e.g. let x = ...)
    Variable(Symbol),

    /// A specific slot in an array (e.g. arr[2])
    /// Tracks the ID of the array and the index accessed
    ArraySlot {
        id: usize,
        index: usize,
    },

    // ArrayLiteral {
    //     id: usize,
    // },

    /// Result of an expression (e.g. 2 + 2, foo())
    Expression,

    /// Just the number 10 or string "hello"
    Literal,

    /// Fallback for initial state
    None, // or Unknown
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct TrackedValue {
    pub value: Value,
    pub source: DataSource,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum Value {
    Integer(i64),
    Float(f64),
    // TODO: maybe differentiate between static strings and dynamic strings
    String(String),
    Boolean(bool),
    Nil,
    Array {
        id: usize,
        elements: Vec<TrackedValue>,
        element_type: Type,
    },
    // Other variants omitted
    // Function, 
}

impl From<LiteralValue> for TrackedValue {
    fn from(lit: LiteralValue) -> Self {
        TrackedValue {
            value: Value::from(lit),
            source: DataSource::Literal,
        }
    }
}

impl From<Value> for TrackedValue {
    fn from(value: Value) -> Self {
        TrackedValue {
            value,
            source: DataSource::None,
        }
    }
}

impl From<LiteralValue> for Value {
    fn from(lit: LiteralValue) -> Self {
        match lit {
            LiteralValue::Integer(n) => Value::Integer(n),
            LiteralValue::Float(n) => Value::Float(n),
            LiteralValue::String(s) => Value::String(s),
            LiteralValue::Boolean(b) => Value::Boolean(b),
            LiteralValue::Nil => Value::Nil,
        }
    }
}

impl TrackedValue {
    pub fn get_type(&self) -> Type {
        match &self.value {
            Value::Integer(_) => Type::Integer,
            Value::Float(_) => Type::Float,
            Value::String(_) => Type::String,
            Value::Boolean(_) => Type::Boolean,
            Value::Nil => Type::Nil,
            Value::Array { id: _, elements: _, element_type } => Type::Array(Box::new(element_type.clone())),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::Array{elements, ..} => {
                let elements: Vec<String> = elements.iter().map(|v| format!("{}", v)).collect();
                write!(f, "[{}]", elements.join(", "))
            },
        }
    }
}

impl std::fmt::Display for TrackedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}