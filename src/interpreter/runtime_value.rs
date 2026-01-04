use crate::interpreter::RuntimeError;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    // Other variants omitted
    // Function, 
}

impl RuntimeValue {
    pub fn add(self, other: &RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
        match (self, other) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::Number(a + b)),
            (RuntimeValue::String(a), RuntimeValue::String(b)) => Ok(RuntimeValue::String(a + &b)),
            _ => Err(RuntimeError::TypeMismatch { expected: "number or string".to_string(), found: "other".to_string() }),
        }
    }

    pub fn subtract(self, other: &RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
        match (self, other) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::Number(a - b)),
            _ => Err(RuntimeError::TypeMismatch { expected: "number".to_string(), found: "other".to_string() }),
        }
    }

    pub fn multiply(self, other: &RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
        match (self, other) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::Number(a * b)),
            _ => Err(RuntimeError::TypeMismatch { expected: "number".to_string(), found: "other".to_string() }),
        }
    }

    pub fn divide(self, other: &RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
        match (self, other) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                if *b == 0.0 {
                    Err(RuntimeError::DivisionByZero())
                } else {
                    Ok(RuntimeValue::Number(a / b))
                }
            },
            _ => Err(RuntimeError::TypeMismatch { expected: "number".to_string(), found: "other".to_string() }),
        }
    }

    pub fn modulus(self, other: &RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
        match (self, other) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => {
                if *b == 0.0 {
                    Err(RuntimeError::DivisionByZero())
                } else {
                    Ok(RuntimeValue::Number(a % b))
                }
            },
            _ => Err(RuntimeError::TypeMismatch { expected: "number".to_string(), found: "other".to_string() }),
        }
    }

    pub fn and(self, other: &RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
        match (self, other) {
            (RuntimeValue::Boolean(a), RuntimeValue::Boolean(b)) => Ok(RuntimeValue::Boolean(a && *b)),
            _ => Err(RuntimeError::TypeMismatch { expected: "boolean".to_string(), found: "other".to_string() }),
        }
    }

    pub fn or(self, other: &RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
        match (self, other) {
            (RuntimeValue::Boolean(a), RuntimeValue::Boolean(b)) => Ok(RuntimeValue::Boolean(a || *b)),
            _ => Err(RuntimeError::TypeMismatch { expected: "boolean".to_string(), found: "other".to_string() }),
        }
    }

    pub fn equals(&self, other: &RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
        match (self, other) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::Boolean(a == b)),
            (RuntimeValue::String(a), RuntimeValue::String(b)) => Ok(RuntimeValue::Boolean(a == b)),
            (RuntimeValue::Boolean(a), RuntimeValue::Boolean(b)) => Ok(RuntimeValue::Boolean(a == b)),
            _ => Err(RuntimeError::TypeMismatch { expected: "same type".to_string(), found: "different types".to_string() }),
        }
    }

    pub fn not_equals(&self, other: &RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
        match (self, other) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::Boolean(a != b)),
            (RuntimeValue::String(a), RuntimeValue::String(b)) => Ok(RuntimeValue::Boolean(a != b)),
            (RuntimeValue::Boolean(a), RuntimeValue::Boolean(b)) => Ok(RuntimeValue::Boolean(a != b)),
            _ => Err(RuntimeError::TypeMismatch { expected: "same type".to_string(), found: "different types".to_string() }),
        }
    }

    pub fn less_than(&self, other: &RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
        match (self, other) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::Boolean(a < b)),
            _ => Err(RuntimeError::TypeMismatch { expected: "number".to_string(), found: "other".to_string() }),
        }
    }

    pub fn less_than_equal(&self, other: &RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
        match (self, other) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::Boolean(a <= b)),
            _ => Err(RuntimeError::TypeMismatch { expected: "number".to_string(), found: "other".to_string() }),
        }
    }

    pub fn greater_than(&self, other: &RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
        match (self, other) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::Boolean(a > b)),
            _ => Err(RuntimeError::TypeMismatch { expected: "number".to_string(), found: "other".to_string() }),
        }
    }

    pub fn greater_than_equal(&self, other: &RuntimeValue) -> Result<RuntimeValue, RuntimeError> {
        match (self, other) {
            (RuntimeValue::Number(a), RuntimeValue::Number(b)) => Ok(RuntimeValue::Boolean(a >= b)),
            _ => Err(RuntimeError::TypeMismatch { expected: "number".to_string(), found: "other".to_string() }),
        }
    }

    pub fn not(self) -> Result<RuntimeValue, RuntimeError> {
        match self {
            RuntimeValue::Boolean(a) => Ok(RuntimeValue::Boolean(!a)),
            _ => Err(RuntimeError::TypeMismatch { expected: "boolean".to_string(), found: "other".to_string() }),
        }
    }

    pub fn negate(self) -> Result<RuntimeValue, RuntimeError> {
        match self {
            RuntimeValue::Number(a) => Ok(RuntimeValue::Number(-a)),
            _ => Err(RuntimeError::TypeMismatch { expected: "number".to_string(), found: "other".to_string() }),
        }
    }
}

impl std::fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeValue::Number(n) => write!(f, "{}", n),
            RuntimeValue::String(s) => write!(f, "{}", s),
            RuntimeValue::Boolean(b) => write!(f, "{}", b),
            RuntimeValue::Nil => write!(f, "nil"),
        }
    }
}