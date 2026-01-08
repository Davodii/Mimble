// use crate::common::{Span, span::Spanned};

// pub type RuntimeError = Spanned<RuntimeErrorKind>;

// #[derive(Debug, Clone)]
// pub enum RuntimeErrorKind {
//     DivisionByZero(),
//     UndefinedVariable(String),
//     TypeMismatch{expected: String, found: String},
//     VariableAlreadyDeclared { var_name: String, def_loc: Span },
//     // Other runtime error variants can be added here
// }

// impl std::fmt::Display for RuntimeErrorKind {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             RuntimeErrorKind::DivisionByZero() => {
//                 write!(f, "division by zero")
//             }
//             RuntimeErrorKind::UndefinedVariable(name) => {
//                 write!(f, "undefined variable '{}' found", name)
//             }
//             RuntimeErrorKind::TypeMismatch{expected, found} => {
//                 write!(
//                     f,
//                     "type mismatch, expected '{}' but found '{}'",
//                     expected, found
//                 )
//             }
//             RuntimeErrorKind::VariableAlreadyDeclared { var_name, def_loc } => {
//                 write!(f, "variable '{}' already declared at {}", var_name, def_loc)
//             }
//         }
//     }
// }