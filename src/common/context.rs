use std::{cell::RefCell, rc::Rc};

use crate::{DiagnosticsSink, common::SymbolPool};


#[derive(Clone)]
pub struct Context {
    pub pool: Rc<RefCell<SymbolPool>>,
    pub diagnostics: Rc<RefCell<DiagnosticsSink>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            pool: Rc::new(RefCell::new(SymbolPool::new())),
            diagnostics: Rc::new(RefCell::new(DiagnosticsSink::new())),
        }
    }
}