
// Define all modules
mod common;
mod lexer;
mod parser;
mod evaluator;
pub mod tracer;

use std::collections::HashMap;

use common::StringPool;
pub use common::DiagnosticsSink;
pub use evaluator::Value;
pub use evaluator::Environment;

use crate::evaluator::TrackedValue;

pub type NativeFn = fn(Vec<TrackedValue>) -> TrackedValue;

#[derive(Clone)]
pub struct NativeRegistry {
    pub functions: HashMap<String, NativeFn>,
}

pub struct Interpreter {
    pool: StringPool,
    sink: DiagnosticsSink,
    globals: Environment,
    tracer: Option<Box<dyn tracer::Tracer>>,
    registry: NativeRegistry,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut registry = HashMap::new();

        // len(arr)
        registry.insert("len".to_string(), (|args: Vec<TrackedValue>|  {
            if let Some(TrackedValue { 
                value: Value::Array { id: _, elements, element_type: _ }, 
                source: _ 
            }) = args.get(0) {
                TrackedValue::from(Value::Integer(elements.len() as i64))
            } else { TrackedValue::from(Value::Nil) }
        }) as fn(Vec<TrackedValue>) -> TrackedValue);


        Interpreter {
            pool: StringPool::new(),
            sink: DiagnosticsSink::new(),
            globals: Environment::new(), // TODO: have a way to change the globals in code
            tracer: None,
            registry: NativeRegistry { functions: registry },
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
            // &mut self.globals, 
            &mut self.pool, 
            &mut self.sink,
            self.tracer.take(),
            self.registry.clone(),
        );
        let result = evaluator.interpret(ast);

        // Restore the tracer
        self.tracer = evaluator.take_tracer();

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
