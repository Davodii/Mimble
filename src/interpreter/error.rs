use crate::Location;

#[derive(Debug, Clone)]
pub enum RuntimeError {
    DivisionByZero(Location),
    UndefinedVariable(String, Location),
    TypeMismatch(String, String, Location),
    // Other runtime error variants can be added here
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::DivisionByZero(loc) => {
                write!(f, "Division by zero at {}", loc)
            }
            RuntimeError::UndefinedVariable(name, loc) => {
                write!(f, "Undefined variable '{}' at {}", name, loc)
            }
            RuntimeError::TypeMismatch(expected, found, loc) => {
                write!(
                    f,
                    "Type mismatch, expected '{}' but found '{}' at {}",
                    expected, found, loc
                )
            }
        }
    }
}