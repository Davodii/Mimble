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

    fn add_native(&mut self, name: &str, args: Vec<Type>, return_type: Type, func: NativeFn) {
        let symbol = self.ctx.pool.borrow_mut().intern(name);
        let value = TrackedValue {
            value: Value::Function(FunctionType::Native {
                name: symbol,
                args,
                return_type,
                func,
            }),
            source: DataSource::Native,
        };
        self.env.define(symbol, value);
    }

    pub fn with_std_io(mut self) -> Self {
        self.add_native("print", vec![Type::Any], Type::Nil, Arc::new(|args| {
            if args.len() != 1 {
                return Err(format!("Expected exactly 1 argument, got {}", args.len()));
            }
            
            Ok(TrackedValue::nil())
        }));
        self
    }

    pub fn with_array(mut self) -> Self {
        self.add_native("len", vec![Type::Array(Box::new(Type::Any))], Type::Integer, Arc::new(|args| {
            if args.len() != 1 {
                return Err(format!("Expected exactly 1 argument, got {}", args.len()));
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
