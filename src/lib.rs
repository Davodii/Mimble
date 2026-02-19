
// Define all modules
mod common;
mod lexer;
mod parser;
pub mod evaluator;
pub mod tracer;
mod stdlib;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use common::SymbolPool;
pub use common::DiagnosticsSink;

use crate::{common::context::Context, evaluator::{environment::Environment, value::Value}};

pub struct Interpreter {
    ctx: Context,
    globals: Rc<RefCell<Environment>>,
    tracer: Option<Box<dyn tracer::Tracer>>,
}

impl Interpreter {
    pub fn new() -> Self {
        // Build the context
        let context = Context::new();

        // Build the standard library
        let globals_builder = stdlib::GlobalsBuilder::new(context.clone());
        let globals = globals_builder
            .with_array()
            .with_std_io()
            .build();

        Interpreter {
            ctx: context,
            globals: globals,
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
        let mut lexer: lexer::Lexer = lexer::Lexer::new(code, self.ctx.clone());
        let tokens = lexer.lex();

        // Parse AST
        let mut parser = parser::Parser::new(tokens, self.ctx.clone());
        let ast = parser.parse();

        // TODO: perform semantic analysis here

        if self.ctx.diagnostics.borrow().has_errors() {
            return Err(());
        }

        // Interpreter
        let mut evaluator = evaluator::WalkerEvaluator::new(
            self.globals.clone(), 
            self.ctx.clone(),
            self.tracer.take()
        );
        let result = evaluator.interpret(ast);

        // Restore the tracer
        self.tracer = evaluator.take_tracer();

        match result {
            Ok(tracked) => Ok(tracked.value), // Unwrap TrackedValue to Value
            Err(_) => return Err(()),
        }
    }

    pub fn emit_diagnostics(&mut self, source: &str) {
        self.ctx.diagnostics.borrow().emit_all(source);
    }

    pub fn clear_diagnostics(&mut self) {
        self.ctx.diagnostics.borrow_mut().clear();
    }
}
