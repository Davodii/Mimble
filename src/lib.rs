use crate::{error::MimbleError, interpreter::RuntimeValue};

// Define all moduless
pub mod error;
pub mod lexer;
pub mod parser;
pub mod interpreter;
mod location;

pub use location::Location;

pub fn run(code: &str) -> Result<RuntimeValue, MimbleError> {
    let mut lexer: lexer::Lexer = lexer::Lexer::new(code);
    let tokens = lexer.lex().map_err(MimbleError::Lex)?;

    // Parse AST
    let mut parser = parser::Parser::new(tokens);
    let statements = parser.parse().map_err(MimbleError::Parse)?;


    // Interpreter
    let mut evaluator = interpreter::WalkerEvaluator::new();
    let result = evaluator.interpret(statements).map_err(MimbleError::Runtime)?;

    Ok(result)
}