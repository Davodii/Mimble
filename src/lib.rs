
// Define all modules
mod common;
mod lexer;
mod parser;
mod interpreter;

use common::StringPool;


pub use common::DiagnosticsSink;
pub use interpreter::RuntimeValue;


pub fn run(code: &str, mut sink: &mut DiagnosticsSink) -> Result<RuntimeValue, ()> {
    // Create the string pool
    let mut pool = StringPool::new();

    let mut lexer: lexer::Lexer = lexer::Lexer::new(code, &mut pool, &mut sink );
    let tokens = lexer.lex();

    // Parse AST
    let mut parser = parser::Parser::new(tokens, &mut pool, &mut sink);
    let ast = parser.parse();

    // TODO: perform semantic analysis here

    if sink.has_errors() {
        return Err(());
    }

    // Interpreter
    let mut evaluator = interpreter::WalkerEvaluator::new(&mut pool, &mut sink);
    let result = evaluator.interpret(ast);

    match result {
        Ok(value) => Ok(value),
        Err(_) => return Err(()),
    }
}