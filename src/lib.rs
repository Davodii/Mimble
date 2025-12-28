
// Define all moduless
mod error;
mod lexer;
mod parser;
mod interpreter;
mod location;

pub use location::Location;
pub use interpreter::RuntimeValue;
pub use error::MimbleError;

#[derive(Debug)]
 pub enum GeneralError {
     Lex(lexer::LexerError),
     Parse(parser::ParserError),
     Runtime(interpreter::RuntimeError),
 }

pub fn run(code: &str) -> Result<RuntimeValue, GeneralError> {
    let mut lexer: lexer::Lexer = lexer::Lexer::new(code);
    let tokens = lexer
        .lex()
        .map_err(GeneralError::Lex)?;

    // Parse AST
    let mut parser = parser::Parser::new(tokens);
    let statements = parser.parse().map_err(GeneralError::Parse)?;

    // Interpreter``
    let mut evaluator = interpreter::WalkerEvaluator::new();
    let result = evaluator
        .interpret(statements)
        .map_err(GeneralError::Runtime)?;

    Ok(result)
}