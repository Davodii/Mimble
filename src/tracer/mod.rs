use std::any::Any;

use crate::evaluator::value::{DataSource, TrackedValue};

#[derive(Debug, Clone)]
pub enum TraceEvent {
    /// For `let x = 10` or `let arr = [1,2,3]`
    Init {
        location: DataSource,
        value: TrackedValue,
    },

    /// For `x = 20` or `arr[0] = 5`
    Assign {
        from: DataSource,
        to: DataSource,
        value: TrackedValue,
    },

    // This is currently only for array index assignments
    // TODO: update to be more general 
    Compare {
        left: DataSource,
        right: DataSource,
        operator: String,
        result: bool,
    },

    BranchEnter {
        statement_id: usize, // Unique ID for the branch statement (e.g., if, match arm)
        condition_result: bool, // Result of the branch condition
    },

    BranchExit {
        statement_id: usize, // Unique ID for the branch statement
    },
}

pub trait Tracer {
    fn trace(&mut self, event: TraceEvent);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// A dummy tracer useful for testing.
pub struct DummyTracer;

impl Tracer for DummyTracer {
    fn trace(&mut self, _event: TraceEvent) {
        // Do nothing
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// A tracer that prints trace events to the console.
pub struct ConsoleTracer;

impl Tracer for ConsoleTracer {
    fn trace(&mut self, event: TraceEvent) {
        println!("[TRACE]: {:?}", event);
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// A tracer for collecting trace events in memory.
pub struct TraceCollector {
    pub events: Vec<TraceEvent>,
}

impl TraceCollector {
    pub fn new() -> Self {
        TraceCollector {
            events: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn get_events(&self) -> &Vec<TraceEvent> {
        &self.events
    }
}

impl Tracer for TraceCollector {
    fn trace(&mut self, event: TraceEvent) {
        self.events.push(event);
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// TODO: link literal expressions to constant ids
// TODO: any assignment or mutation should be traceable 
//       to the variable or data structure being modified