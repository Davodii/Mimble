use crate::Location;

#[derive(Debug, Clone)]
pub enum ParserError {
    UnexpectedToken(String, Location),
    UnexpectedEOF,
    // Other parser error variants can be added here
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedToken(token, loc) => {
                write!(f, "Unexpected token '{}' at {}", token, loc)
            }
            ParserError::UnexpectedEOF => {
                write!(f, "Unexpected end of file")
            }
        }
    }
}