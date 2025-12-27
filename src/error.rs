use crate::lexer::LexerError;
use crate::parser::ParserError;
use crate::interpreter::RuntimeError;

#[derive(Debug)]
pub enum MimbleError {
    Lex(LexerError),
    Parse(ParserError),
    Runtime(RuntimeError),
    // IO(std::io::Error), // Example of wrapping another error type
}

impl std::fmt::Display for MimbleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MimbleError::Lex(err) => write!(f, "[Lexing Error] {}", err),
            MimbleError::Parse(err) => write!(f, "[Parsing Error] {}", err),
            MimbleError::Runtime(err) => write!(f, "[Runtime Error] {}", err),
            // MimbleError::IO(err) => write!(f, "IO Error: {}", err),
        }
    }
}

impl std::error::Error for MimbleError {}