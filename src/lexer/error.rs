// use crate::Span;

// #[derive(Debug, Clone)]
// pub struct LexerError {
//     pub location: Span,
//     pub kind: LexerErrorKind,
// }

// #[derive(Debug, Clone)]
// pub enum LexerErrorKind {
//     InvalidCharacter { character: char },
//     UnterminatedString,
//     UnexpectedEOF,
// }

// impl LexerError {
//     pub fn to_diagnostic(&self) -> crate::diagnostics::Diagnostic {
//         let message = match &self.kind {
//             LexerErrorKind::InvalidCharacter { character } => {
//                 format!("invalid character '{}'", character)
//             }
//             LexerErrorKind::UnterminatedString => {
//                 "unterminated string literal".to_string()
//             }
//             LexerErrorKind::UnexpectedEOF => {
//                 "unexpected end of file".to_string()
//             }
//         };

//         crate::diagnostics::Diagnostic {
//             message,
//             location: self.location,
//             severity: crate::diagnostics::Severity::Error,
//         }
//     }
// }