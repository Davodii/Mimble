// Lexer for the language
mod token;
mod lexer;
mod error;

pub use token::{Token, TokenKind};
pub use lexer::Lexer;
pub use error::LexerError;