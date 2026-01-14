use super::environment::Environment;
use super::value::Value;
use crate::evaluator::DataSource;
use crate::evaluator::value::TrackedValue;
use crate::parser::{Stmt, StmtKind, Expr, ExprKind};

use crate::common::{DiagnosticsSink, Span, StringPool, Symbol};
use crate::lexer::TokenKind;
use crate::tracer::TraceEvent;

#[derive(Copy, Clone)]
enum BinOpKind {
    Numeric,
    NumericOrStringConcat,
    Equality,
    Ordering,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum NumericKind {
    Integer,
    Float,
}

fn promote_numeric(lhs: Value, rhs: Value) -> Result<(Value, Value, NumericKind), ()> {
    use NumericKind::*;
    
    let lk = match lhs {
        Value::Integer(_) => Integer,
        Value::Float(_) => Float,
        _ => return Err(()),
    };

    let rk = match rhs {
        Value::Integer(_) => Integer,
        Value::Float(_) => Float,
        _ => return Err(()),
    };

    let target = if lk == Float || rk == Float { Float } else { Integer };

    let lhs = match (lhs, target) {
        (Value::Integer(n), Float) => Value::Float(n as f64),
        (v, _) => v,
    };

    let rhs = match (rhs, target) {
        (Value::Integer(n), Float) => Value::Float(n as f64),
        (v, _) => v,
    };

    Ok((lhs, rhs, target))
}

fn numeric_binop<TInt, TFloat>(
    lhs: Value,
    rhs: Value,
    int_op: TInt,
    float_op: TFloat,
) -> Result<Value, ()>
where 
    TInt: FnOnce(i64, i64) -> Value,
    TFloat: FnOnce(f64, f64) -> Value,
{
    let (lhs, rhs, kind) = promote_numeric(lhs, rhs)?;

    Ok(match kind {
        NumericKind::Integer => {
            let (Value::Integer(l), Value::Integer(r)) = (lhs, rhs) else { unreachable!() };
            int_op(l, r)
        },
        NumericKind::Float => {
            let (Value::Float(l), Value::Float(r)) = (lhs, rhs) else { unreachable!() };
            float_op(l, r)
        },
    })
}

fn concat(lhs: Value, rhs: Value) -> Result<Value, ()> {
    match (lhs, rhs) {
        (Value::String(a), Value::String(b)) => Ok(Value::String(a + &b)),
        (Value::String(a), b) => Ok(Value::String(a + &b.to_string())),
        (a, Value::String(b)) => Ok(Value::String(a.to_string() + &b)),
        _ => Err(()),
    }
}

fn equals(lhs: Value, rhs: Value) -> Result<Value, ()> {
    Ok(Value::Boolean(match (&lhs, &rhs) {
        (Value::Integer(a), Value::Integer(b)) => a == b,
        (Value::Float(a), Value::Float(b)) => a == b,
        (Value::Integer(a), Value::Float(b)) => (*a as f64) == *b,
        (Value::Float(a), Value::Integer(b)) => *a == (*b as f64),
        (Value::String(a), Value::String(b)) => a == b,
        (Value::Boolean(a), Value::Boolean(b)) => a == b,
        (Value::Nil, Value::Nil) => true,
        _ => false,
    }))
}

fn ordering_cmp<FInt, FFloat>(
    lhs: Value,
    rhs: Value,
    int_cmp: FInt,
    float_cmp: FFloat,
) -> Result<Value, ()>
where
    FInt: FnOnce(i64, i64) -> bool,
    FFloat: FnOnce(f64, f64) -> bool,
{
    numeric_binop(
        lhs, 
        rhs, 
        |a, b| Value::Boolean(int_cmp(a, b)), 
        |a, b| Value::Boolean(float_cmp(a, b))
    )
}

fn eval_binop(op: TokenKind, lhs: Value, rhs: Value) -> Result<Value, ()> {
    match op {
        TokenKind::Plus => match (&lhs, &rhs) {
            (Value::String(_), _) | (_, Value::String(_)) => concat(lhs, rhs),
            _ => numeric_binop(lhs, rhs, |a, b| Value::Integer(a + b), |a, b| Value::Float(a + b)),
        },

        TokenKind::Minus    => numeric_binop(lhs, rhs, |a, b| Value::Integer(a - b), |a, b| Value::Float(a - b)),
        TokenKind::Star     => numeric_binop(lhs, rhs, |a, b| Value::Integer(a * b), |a, b| Value::Float(a * b)),
        TokenKind::Slash    => numeric_binop(lhs, rhs, |a, b| Value::Integer(a / b), |a, b| Value::Float(a / b)),
        
        TokenKind::EQ       => equals(lhs, rhs),
        TokenKind::NEQ      => equals(lhs, rhs).map(|v| Value::Boolean(!matches!(v, Value::Boolean(false)))),

        TokenKind::LT       => ordering_cmp(lhs, rhs, |a, b| a < b, |a, b| a < b),
        TokenKind::LEQ      => ordering_cmp(lhs, rhs, |a, b| a <= b, |a, b| a <= b),
        TokenKind::GT       => ordering_cmp(lhs, rhs, |a, b| a > b, |a, b| a > b),
        TokenKind::GEQ      => ordering_cmp(lhs, rhs, |a, b| a >= b, |a, b| a >= b),

        _ => Err(()),
    }
}
pub struct WalkerEvaluator<'a> {
    // fields omitted
    environment: &'a mut Environment,
    pool: &'a mut StringPool,
    sink: &'a mut DiagnosticsSink,

    next_uid: usize,
}

impl<'a> WalkerEvaluator<'a> {
    pub fn new(env: &'a mut Environment, pool: &'a mut StringPool, sink: &'a mut DiagnosticsSink) -> Self {
        Self {
            environment: env,
            pool,
            sink,
            next_uid: 0,
        }
    }

    pub fn interpret(&mut self, code: Vec<Stmt>) -> Result<TrackedValue, ()> {
        let mut value = TrackedValue::from(Value::Nil);
        
        for stmt in code {
            match self.execute_statement(&stmt) {
                Ok(val) => value = val,
                Err(_) => {
                    // An error has already been reported
                    return Err(());
                }
            }
        }

        Ok(value)
    }

    fn new_uid(&mut self) -> usize {
        let id = self.next_uid;
        self.next_uid += 1;
        id
    }

    fn error(&mut self, span: Span, message: impl Into<String>){
        self.sink.report(
            span, 
            message.into(),
            crate::common::Severity::Error,
        );
    }

    fn execute_statement(&mut self, stmt: &Stmt) -> Result<TrackedValue, ()> {
        match stmt {
            Stmt{
                node: StmtKind::ExprStmt(expr), 
                span: _
            } => {
                self.evaluate_expression(&expr)
            },
            Stmt{
                node: StmtKind::LetStmt { 
                    name, 
                    type_annotation, 
                    initializer 
                }, 
                span: _
            } => self.execute_let_statement(name, type_annotation, initializer),
        }
    }

    fn evaluate_expression(&mut self, expr: &crate::parser::Expr) -> Result<TrackedValue, ()> {
        match expr {
            Expr{node: ExprKind::Literal(lit), span: _} => Ok(TrackedValue::from(lit.clone())),
            Expr{node: ExprKind::Binary { left, op, right }, span: _} => {
                self.execute_binary_operation(left, op, right)
            },
            Expr{node: ExprKind::Unary { op, expr }, span: _} => {
                self.evaluate_unary(op, expr)
            },
            Expr{node: ExprKind::Identifier(name), span } => {
                if let Some(value) = self.environment.get(name) {
                    Ok(value)
                } else {
                    self.error(
                        *span, 
                        format!(
                            "undefined variable '{}' found", 
                            self.pool.resolve(*name)
                        )
                    );
                    Err(())
                }
            },
            Expr{node: ExprKind::Assign { name, value }, span} => {
                self.execute_variable_assignment_statement(name, value, *span)
            },
            Expr{node: ExprKind::ArrayLiteral(elements), span: _} => {
                self.execute_array_literal(elements)
            },
            Expr{node: ExprKind::Get { object, index }, span: _} => {
                self.execute_array_get(object, index)
            },
            Expr{node: ExprKind::Set { object, index, value }, span: _} => {
                self.execute_array_set(object, index, value)
            },
        }
    }

    fn evaluate_unary(&mut self, op: &TokenKind, expr: &Box<crate::common::span::Spanned<ExprKind>>) -> Result<TrackedValue, ()> {
        let tv = self.evaluate_expression(expr)?;
        let value = match op {
            TokenKind::Minus => {
                match &tv.value {
                    Value::Integer(n) => Value::Integer(-n),
                    Value::Float(n) => Value::Float(-n),
                    _ => {
                        self.error(
                            expr.span, 
                            "unary '-' operator requires numeric operand"
                        );
                        return Err(())
                    }
                }
            },
            TokenKind::Not => {
                match &tv.value {
                    Value::Boolean(b) => Value::Boolean(!b),
                    _ => {
                        self.error(
                            expr.span, 
                            "unary 'not' operator requires boolean operand"
                        );
                        return Err(())
                    }
                }
            },
            _ => todo!("Handle type mismatches"),
        };
    
        Ok(TrackedValue::from(value))
    }
    
    fn execute_binary_operation(&mut self, left: &Box<Expr>, op: &TokenKind, right: &Box<Expr>) -> Result<TrackedValue, ()> {
        // Evaluate the left and right expressions
        let left_val = self.evaluate_expression(left)?;
        let right_val = self.evaluate_expression(right)?;


        let result_value = eval_binop(
            op.clone(), 
            left_val.value, 
            right_val.value
        )?;
        Ok(TrackedValue::from(result_value))
    }

    fn execute_let_statement(
        &mut self, name: &Symbol, 
        type_annotation: &Option<crate::common::Type>, 
        initializer: &Box<Expr>
    ) -> Result<TrackedValue, ()> {
        // Evaluate the initializer expression
        let value = self.evaluate_expression(initializer)?;

        // If we have a type annotation, check that the value matches the type
        if let Some(expected_type) = type_annotation {
            let value_type = value.get_type();
            if &value_type != expected_type {
                self.error(
                    initializer.span, 
                    format!(
                        "Variable was declared with type '{}' but expression has type '{}'",
                        expected_type,
                        value_type
                    ),
                );

                return Err(());
            }
        }

        // Define the destination identity
        let destination = DataSource::Variable(*name);

        // TODO: Emit init event
        // Even though it is a new variable, we show the data moving
        // FROM its source INTO its new home
        // self.emit(TraceEvent::Assign {
        //     from: value.source.clone(),
        //     to: destination.clone(),
        //     value: value.value.clone(),
        // });

        // Update the source of the value to be the variable itself
        let tracked_for_env = TrackedValue {
            value: value.value.clone(),
            source: destination,
        };

        self.environment.define(name.clone(), tracked_for_env.clone());
        Ok(value)
    }

    fn execute_variable_assignment_statement(
        &mut self, 
        name: &Symbol, 
        value: &Box<Expr>,
        span: Span,
    ) -> Result<TrackedValue, ()> {
        // Evaluate the expression
        let val = self.evaluate_expression(value)?;

        // Define the destination
        let destination = DataSource::Variable(*name);

        // TODO: Emit the trace event
        // self.emit(TraceEvent::Assing {
        //     from: val.source.clone(),
        //     to: destination.clone(),
        //     value: val.value.clone(),
        // });

        // Since the value now lives in the variable, we update its source
        // so that the next time it is moved, it reports this variable as its origin
        let updated_val = TrackedValue {
            value: val.value.clone(),
            source: destination,
        };

        if self.environment.assign(name, updated_val.clone()) {
            Ok(updated_val)
        } else {
            self.error(
                span, 
                format!(
                    "undefined variable '{}' found", 
                    self.pool.resolve(*name)
                )
            );
            Err(())
        }
    }

    fn execute_array_literal(&mut self, elements: &Vec<Expr>) -> Result<TrackedValue, ()> {
        let mut values = Vec::new();
        let mut element_type: crate::common::Type = crate::common::Type::Nil;
        for elem in elements {
            let val = self.evaluate_expression(elem)?;

            if element_type != crate::common::Type::Nil {
                if val.get_type() != element_type {
                    self.error(
                        elem.span, 
                        format!(
                            "Array literal has inconsistent element types: expected '{:?}', found '{:?}'",
                            element_type,
                            val.get_type()
                        )
                    );
                    return Err(());
                }
            } else {
                element_type = val.get_type();
            }

            values.push(val);
        }
        Ok(
            TrackedValue { 
                value: Value::Array{
                    id: self.new_uid(),
                    elements: values,
                    element_type,
                }, 
                source: DataSource::Expression,
            } 
        )
    }

    fn execute_array_get(&mut self, object: &Box<Expr>, index: &Box<Expr>) -> Result<TrackedValue, ()> {
        let object_val = self.evaluate_expression(object)?;
        let index_val = self.evaluate_expression(index)?;

        match (&object_val.value, &index_val.value) {
            (Value::Array{id, elements: arr, element_type: _}, Value::Integer(n)) => {
                let idx = *n as usize;
                match arr.get(idx) {
                    Some(val) => {
                        Ok(
                            TrackedValue {
                                value: val.value.clone(),
                                source: DataSource::ArraySlot { 
                                    id: *id, 
                                    index: idx 
                                }
                            }
                        )
                    },
                    None => {
                        self.error(
                            index.span, 
                            "array index out of bounds");
                        Err(())
                    }
                }
            },
            _ => {
                self.error(
                    index.span, 
                    "expected array with an integer index");
                Err(())
            },
        }
    }

    fn execute_array_set(&mut self, obj_expr: &Box<Expr>, idx_expr: &Box<Expr>, val_expr: &Box<Expr>) -> Result<TrackedValue, ()> {
        // Evaluate all parts
        let object = self.evaluate_expression(obj_expr)?;
        let index = self.evaluate_expression(idx_expr)?;
        let value = self.evaluate_expression(val_expr)?;

        if let (Value::Array { id: array_id, elements, element_type}, Value::Integer(n)) = (&object.value, &index.value) {
            let idx = *n as usize;

            if idx >= elements.len() {
                self.error(
                    idx_expr.span, 
                    "array index out of bounds"
                );
                return Err(());
            }

            // Check type compatibility
            if value.get_type() != *element_type {
                self.error(
                    val_expr.span, 
                    format!(
                        "Array only accepts elements of type '{:?}', but found '{:?}'",
                        element_type,
                        value.get_type()
                    )
                );
                return Err(());
            }

            // Trace event
            // We know the source of value (the 'from')
            // We know the destination (ArraySlot {id: array_id, index: idx})
            let target_source = DataSource::ArraySlot { id: *array_id, index: idx };

            // TODO: implement the trait!!!
            // self.emit(TraceEvent::Assign { 
            //     from: value.source.clone(), 
            //     to: target_source.clone(), 
            //     value: value.value.clone() 
            // });

            // Internal state update
            // We update the elements. Note that the value stored in the array
            // now considers ITS source to be that specifc array slot
            let mut new_elements = elements.clone();
            new_elements[idx] = TrackedValue {
                value: value.value.clone(),
                source: target_source,
            };

            let new_array_value = Value::Array {
                id: *array_id,
                elements: new_elements,
                element_type: element_type.clone(),
            };

            // Persist to environment
            // If the object was a variable, we update it
            if let ExprKind::Identifier(symbol) = obj_expr.node {
                let update_array_tracked = TrackedValue {
                    value: new_array_value,
                    source: DataSource::Variable(symbol),
                };

                if !self.environment.assign(&symbol, update_array_tracked) {
                    self.error(
                        obj_expr.span, 
                        format!(
                            "undefined variable '{}' found", 
                            self.pool.resolve(symbol)
                        )
                    );
                    return Err(());
                }
            }

            Ok(value)
        } else {
            self.error(
                obj_expr.span, 
                "expected array with an integer index"
            );
            Err(())
        }
    }
}
