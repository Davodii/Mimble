use crate::Location;
use crate::error::MimbleError;

#[derive(Debug, Clone)]
pub enum LexerErrorKind {
    InvalidCharacter(char, Location),
    UnterminatedString(Location),
    // Other lexer error variants can be added here
}

pub type LexerError = MimbleError<LexerErrorKind>;

impl std::fmt::Display for LexerErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerErrorKind::InvalidCharacter(c, loc) => {
                write!(f, "Invalid character '{}' at {}", c, loc)
            }
            LexerErrorKind::UnterminatedString(loc) => {
                write!(f, "Unterminated string at {}", loc)
            }
        }
    }
}