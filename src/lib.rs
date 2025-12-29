
// Define all moduless
mod diagnostics;
mod lexer;
mod parser;
mod interpreter;
mod location;

pub use location::Location;
pub use interpreter::RuntimeValue;
pub use diagnostics::DiagnosticsSink;

pub fn run(code: &str) -> Result<RuntimeValue, ()> {
    // Create Reporter
    let mut sink = DiagnosticsSink::new();

    let mut lexer: lexer::Lexer = lexer::Lexer::new(&mut sink, code);
    let tokens = match lexer.lex() {
        Ok(tokens) => tokens,
        Err(_) => {
            // Lexing error occurred
            // Print the diagnostics and return
            sink.emit();
            return Err(());
        },
    };

    // Parse AST
    let mut parser = parser::Parser::new(tokens, &mut sink);
    let program = match parser.parse() {
        Ok(program) => program,
        Err(e) => {
            // The parser encountered a fatal error
            // Print the diagnostics and return
            sink.emit();
            return Err(());
        },
    };

    // Interpreter
    let mut evaluator = interpreter::WalkerEvaluator::new();
    let result = evaluator.interpret(program);

    match result {
        Ok(value) => Ok(value),
        Err(_) => {
            // Runtime error occurred
            sink.emit();
            return Err(());
        },
    }
}