use crate::Location;

#[derive(Debug, Clone)]
pub enum LexerError {
    InvalidCharacter(char, Location),
    UnterminatedString(Location),
    // Other lexer error variants can be added here
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::InvalidCharacter(c, loc) => {
                write!(f, "Invalid character '{}' at {}", c, loc)
            }
            LexerError::UnterminatedString(loc) => {
                write!(f, "Unterminated string at {}", loc)
            }
        }
    }
}