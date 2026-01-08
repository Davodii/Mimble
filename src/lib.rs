
// Define all modules
mod common;
mod lexer;
mod parser;
mod evaluator;

use common::StringPool;


pub use common::DiagnosticsSink;
pub use evaluator::RuntimeValue;
pub use evaluator::Environment;

pub struct Interpreter {
    pool: StringPool,
    sink: DiagnosticsSink,
    globals: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            pool: StringPool::new(),
            sink: DiagnosticsSink::new(),
            globals: Environment::new(), // TODO: have a way to change the globals in code
        }
    }

    pub fn run(&mut self, code: &str) -> Result<RuntimeValue, ()> {

        let mut lexer: lexer::Lexer = lexer::Lexer::new(code, &mut self.pool, &mut self.sink );
        let tokens = lexer.lex();

        // Parse AST
        let mut parser = parser::Parser::new(tokens, &mut self.pool, &mut self.sink);
        let ast = parser.parse();

        // TODO: perform semantic analysis here

        if self.sink.has_errors() {
            return Err(());
        }

        // Interpreter
        let mut evaluator = evaluator::WalkerEvaluator::new(
            &mut self.globals, 
            &mut self.pool, 
            &mut self.sink
        );
        let result = evaluator.interpret(ast);

        match result {
            Ok(value) => Ok(value),
            Err(_) => return Err(()),
        }
    }

    pub fn emit_diagnostics(&mut self, source: &str) {
        self.sink.emit_all(source);
    }

    pub fn clear_diagnostics(&mut self) {
        self.sink = DiagnosticsSink::new();
    }
}
