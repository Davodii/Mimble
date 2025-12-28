use crate::{Location, MimbleError};

#[derive(Debug, Clone)]
pub enum ParserErrorKind {
    UnexpectedToken{ token: String, expected: String},
    UnexpectedEOF,
    // Other parser error variants can be added here
}

pub type ParserError = MimbleError<ParserErrorKind>;

impl std::fmt::Display for ParserErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserErrorKind::UnexpectedToken{ token, expected } => {
                write!(f, "Expected '{}', found '{}'", expected, token)
            }
            ParserErrorKind::UnexpectedEOF => {
                write!(f, "Unexpected end of file")
            }
        }
    }
}