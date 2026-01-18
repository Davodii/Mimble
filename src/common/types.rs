#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum Type {
    Integer,
    Float,
    Boolean,
    String,
    Array(Box<Type>),
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
            Type::Nil => write!(f, "nil"),
        }
    }
}