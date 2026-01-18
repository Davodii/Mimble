
// Define all modules
mod common;
mod lexer;
mod parser;
mod evaluator;
pub mod tracer;

use common::StringPool;


pub use common::DiagnosticsSink;
pub use evaluator::Value;
pub use evaluator::Environment;

pub struct Interpreter {
    pool: StringPool,
    sink: DiagnosticsSink,
    globals: Environment,
    tracer: Option<Box<dyn tracer::Tracer>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            pool: StringPool::new(),
            sink: DiagnosticsSink::new(),
            globals: Environment::new(), // TODO: have a way to change the globals in code
            tracer: None,
        }
    }

    pub fn set_tracer(&mut self, tracer: Box<dyn tracer::Tracer>) {
        self.tracer = Some(tracer);
    }

    pub fn take_tracer(&mut self) -> Option<Box<dyn tracer::Tracer>> {
        self.tracer.take()
    }

    pub fn run(&mut self, code: &str) -> Result<Value, ()> {

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
            &mut self.sink,
            self.tracer.take(),
        );
        let result = evaluator.interpret(ast);

        match result {
            Ok(value) => Ok(value.value), // Unwrap TrackedValue to Value
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
