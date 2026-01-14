use crate::{Value, evaluator::DataSource};

#[derive(Debug, Clone)]
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
}

/// A dummy tracer useful for testing.
#[derive(Debug, Clone, Copy)]
pub struct DummyTracer {}

impl Tracer for DummyTracer {
    fn trace(&mut self, _event: TraceEvent) {
        // Do northing
    }
}

// TODO: link literal expressions to constant ids
// TODO: any assignment or mutation should be traceable 
//       to the variable or data structure being modified