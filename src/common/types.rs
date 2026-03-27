#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum Type {
    Integer,
    Float,
    Boolean,
    String,
    Array(Box<Type>),
    Function { param_types: Vec<Type>, return_type: Box<Type> },
    Nil,
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
            // Type::Undefined => write!(f, "undefined"),
        }
    }
}