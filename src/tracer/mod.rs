use std::any::Any;

use crate::{Value, evaluator::DataSource};

#[derive(Debug, Clone, serde::Serialize)]
pub enum TraceEvent {
    /// For `let x = 10` or `let arr = [1,2,3]`
    Init {
        location: DataSource,
        value: Value,
    },

    /// For `x = 20` or `arr[0] = 5`
    Assign {
        from: DataSource,
        to: DataSource,
        value: Value,
    },

    // This is currently only for array index assignments
    // TODO: update to be more general 
    Compare {
        left: DataSource,
        right: DataSource,
        operator: String,
        result: bool,
    }
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