// use crate::{lexer::TokenKind, diagnostics::Diagnostic};


// #[derive(Debug, Clone)]
// pub struct ParserError {
//     pub location: crate::Span,
//     pub kind: ParserErrorKind,
// }

// #[derive(Debug, Clone)]
// pub enum ParserErrorKind {
//     UnexpectedToken { 
//         expected: TokenKind, 
//         found: TokenKind 
//     },
//     UnexpectedEOF {
//         expected: TokenKind,
//     },
//     InvalidExpression,
//     ExpectedTypeAnnotation { found: TokenKind },
//     UnterminatedBlock {
//         block_name: String, // "do", "if", etc.
//     },
// }

// impl ParserError {
//     pub fn to_diagnostic(&self) -> Diagnostic {
//         let message = match &self.kind {
//             ParserErrorKind::UnexpectedToken { expected, found } => {
//                 format!(
//                     "unexpected token {:?}, expected {:?}",
//                     found,
//                     expected,
//                 )
//             }
//             ParserErrorKind::UnexpectedEOF { expected } => {
//                 format!(
//                     "unexpected end of file, expected {:?}",
//                     expected,
//                 )
//             },
//             ParserErrorKind::InvalidExpression => {
//                 "invalid expression".to_string()
//             },
//             ParserErrorKind::ExpectedTypeAnnotation { found } => {
//                 format!(
//                     "expected type annotation ('int', 'float', etc.), found {:?}",
//                     found,
//                 )
//             },
//             ParserErrorKind::UnterminatedBlock { block_name } => {
//                 format!(
//                     "unterminated block, expected 'end' for '{}'",
//                     block_name,
//                 )
//             },
//         };

//         Diagnostic {
//             message,
//             location: self.location,
//             severity: crate::diagnostics::Severity::Error,
//         }
//     }
// }