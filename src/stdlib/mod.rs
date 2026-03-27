use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use crate::common::Type;
use crate::Value;
use crate::common::context::Context;
use crate::evaluator::value::{FunctionType, TrackedValue, DataSource};
use crate::evaluator::environment::{Environment};

pub type NativeFn = Arc<dyn Fn(Vec<TrackedValue>) -> Result<TrackedValue, String> + Send + Sync>;

/// Module to load in different functionality into an environment
pub struct GlobalsBuilder {
    env: Environment,
    ctx: Context,
}

impl GlobalsBuilder {
    pub fn new(ctx: Context) -> Self {
        GlobalsBuilder {
            env: Environment::new(),
            ctx,
        }
    }

    fn add_native(&mut self, name: &str, return_type: Type, func: NativeFn) {
        let symbol = self.ctx.pool.borrow_mut().intern(name);
        let value = TrackedValue {
            value: Value::Function(FunctionType::Native {
                name: symbol,
                return_type,
                func,
            }),
            source: DataSource::Native,
        };
        self.env.define(symbol, value);
    }

    pub fn with_std_io(mut self) -> Self {
        self.add_native("print", Type::Nil, Arc::new(|args| {
            for arg in args {
                print!("{} ", arg.value);
            }
            println!();
            Ok(TrackedValue::nil())
        }));
        self
    }

    pub fn with_array(mut self) -> Self {
        // TODO: use the diagnostics system to report errors instead of returning Err
        self.add_native("len", Type::Integer, Arc::new(|args| {
            if args.len() != 1 {
                return Err(format!("Expected 1 argument, got {}", args.len()));
            }
            let element_type = args[0].get_type();
            match &args[0].value {
                Value::Array { id: _, elements, element_type: _ } => Ok(TrackedValue {
                    value: Value::Integer(elements.len() as i64),
                    source: DataSource::Expression,
                }),
                _ => Err(format!("Expected an array, got {}", element_type)),
            }
        }));
        self
    }

    pub fn build(self) -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(self.env))
    }
}
