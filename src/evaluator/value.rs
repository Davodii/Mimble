use crate::{common::{Symbol, Type}, parser::{LiteralValue, Stmt}, stdlib::NativeFn};

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(tag = "kind", content = "value")]
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

    /// A function defined in Rust and exposed to mimble code
    Native,

    /// Fallback for initial state
    None, // or Unknown
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackedValue {
    pub value: Value,
    pub source: DataSource,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(tag = "kind", content = "value")]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    #[serde(rename_all = "camelCase")]
    Array {
        id: usize,
        elements: Vec<TrackedValue>,
        element_type: Type,
    },
    Function(FunctionType),
    Nil,
}

#[derive(Clone, serde::Serialize)]
#[serde(tag = "kind", content = "value")]
pub enum FunctionType {
    Native {
        name: Symbol,
        return_type: Type,
        #[serde(skip)]
        func: NativeFn,
    },
    /// Function defined in mimble code
    User {
        name: Symbol,
        return_type: Type,
        params: Vec<(Symbol, Option<Type>)>,
        #[serde(skip)]
        body: Box<Stmt>,
    }
}

impl std::fmt::Debug for FunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionType::Native { name, .. } => write!(f, "NativeFunction({})", name),
            FunctionType::User { name, params, .. } => write!(f, "UserFunction({}, params: {:?})", name, params),
        }
    }
}

impl PartialEq for FunctionType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Native { name: n1, return_type: rt1, .. }, Self::Native { name: n2, return_type: rt2, .. }) => n1 == n2 && rt1 == rt2,
            (Self::User { name: n1, params: p1, body: b1, return_type: rt1 }, Self::User { name: n2, params: p2, body: b2, return_type: rt2 }) => {
                n1 == n2 && p1 == p2 && b1 == b2 && rt1 == rt2
            }
            _ => false,
        }
    }
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
            Value::Function(function_type) => {
                match function_type {
                    FunctionType::Native { name: _, return_type, func: _ } => return_type.clone(),
                    FunctionType::User { name: _, params: _, body: _, return_type } => return_type.clone(),
                    // _ => todo!(),
                }
            },
            // _ => todo!(),
        }
    }

    pub fn nil() -> Self {
        TrackedValue {
            value: Value::Nil,
            source: DataSource::None,
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
            Value::Function(function_type) => {
                match function_type {
                    FunctionType::Native { name, .. } => write!(f, "<native function {}>", name),
                    FunctionType::User { name, params, .. } => {
                        let params: Vec<String> = params.iter().map(|(n, t)| {
                            if let Some(t) = t {
                                format!("{}: {}", n, t)
                            } else {
                                format!("{}", n)
                            }
                        }).collect();
                        write!(f, "<function {}({})>", name, params.join(", "))
                    },
                }
            },
            _ => todo!(),
        }
    }
}

impl std::fmt::Display for TrackedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}