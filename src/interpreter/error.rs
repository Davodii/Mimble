use crate::Location;

#[derive(Debug, Clone)]
pub enum RuntimeError {
    DivisionByZero(),
    UndefinedVariable(String),
    TypeMismatch{expected: String, found: String},
    VariableAlreadyDeclared { var_name: String, def_loc: Location },
    // Other runtime error variants can be added here
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::DivisionByZero() => {
                write!(f, "Division by zero")
            }
            RuntimeError::UndefinedVariable(name) => {
                write!(f, "Undefined variable '{}'", name)
            }
            RuntimeError::TypeMismatch{expected, found} => {
                write!(
                    f,
                    "Type mismatch, expected '{}' but found '{}'",
                    expected, found
                )
            }
            RuntimeError::VariableAlreadyDeclared { var_name, def_loc } => {
                write!(f, "Variable '{}' already declared at {}", var_name, def_loc)
            }
        }
    }
}