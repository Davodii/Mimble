#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum Type {
    Integer,
    Float,
    Boolean,
    String,
    Array(Box<Type>),
    Function { param_types: Vec<Type>, return_type: Box<Type> },
    Nil,
    Any, // Used for native functions that can accept any type of argument (e.g. print)
    // Undefined, // Used for variables that are declared but not yet assigned a value
}

impl Type {
    /// Returns true if a value of type `self` can be used where `other` is expected.
    /// Any type is compatible with everything — it opts out of static checking.
    pub fn is_compatible_with(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Any, _) | (_, Type::Any) => true,
            (Type::Array(a), Type::Array(b)) => a.is_compatible_with(b),
            _ => self == other,
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Integer => write!(f, "integer"),
            Type::Float => write!(f, "float"),
            Type::Boolean => write!(f, "boolean"),
            Type::String => write!(f, "string"),
            Type::Array(elem_type) => write!(f, "array<{}>", elem_type),
            Type::Function { param_types, return_type } => {
                let params = param_types.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", ");
                write!(f, "function({}) -> {}", params, return_type)
            }
            Type::Nil => write!(f, "nil"),
            Type::Any => write!(f, "any"),
            // Type::Undefined => write!(f, "undefined"),
        }
    }
}