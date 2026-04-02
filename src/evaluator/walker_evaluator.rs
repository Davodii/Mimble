use std::cell::RefCell;
use std::rc::Rc;

use super::environment::Environment;
use super::value::Value;
use crate::common::context::Context;
use crate::evaluator::value::{DataSource, FunctionType, TrackedValue};
use crate::parser::{Stmt, StmtKind, Expr, ExprKind};

use crate::common::{Span, Symbol, Type};
use crate::lexer::TokenKind;
use crate::tracer::{TraceEvent, Tracer};

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
        (Value::String(a), Value::String(b)) => {
            Ok(Value::String(a + &b))
        },
        (Value::String(a), b) => {
            Ok(Value::String(a + &b.to_string()))
        },
        (a, Value::String(b)) => {
            Ok(Value::String(a.to_string() + &b))
        },
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
pub struct WalkerEvaluator {
    // fields omitted
    env: Rc<RefCell<Environment>>,
    ctx: Context,
    tracer: Option<Box<dyn Tracer>>,
    next_uid: usize,
    break_hit: bool,
    return_hit: bool,
    continue_hit: bool,
}
impl WalkerEvaluator {
    pub fn new(
        env: Rc<RefCell<Environment>>,
        ctx: Context,
        tracer: Option<Box<dyn Tracer>>,
    ) -> Self {
        Self {
            env,
            ctx,
            tracer,
            next_uid: 0,
            break_hit: false,
            return_hit: false,
            continue_hit: false,
        }
    }

    pub fn set_tracer(&mut self, tracer: Box<dyn Tracer>) {
        self.tracer = Some(tracer);
    }

    pub fn take_tracer(&mut self) -> Option<Box<dyn Tracer>> {
        self.tracer.take()
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

    fn emit(&mut self, event: TraceEvent) {
        if let Some(tracer) = &mut self.tracer {
            tracer.trace(event);
        }
    }

    fn error(&mut self, span: Span, message: impl Into<String>){
        self.ctx.diagnostics.borrow_mut().report(
            span, 
            message.into(),
            crate::common::Severity::Error,
        );
    }

    fn resolve_symbol(&self, symbol: Symbol) -> String {
        self.ctx.pool.borrow().resolve(symbol).to_string()
    }

    fn execute_statement(&mut self, stmt: &Stmt) -> Result<TrackedValue, ()> {
        match &stmt.node {
            StmtKind::ExprStmt(expr) => self.evaluate_expression(&expr),
            StmtKind::LetStmt { name, type_annotation, initializer } => {
                self.execute_let_statement(name, type_annotation, initializer)
            },
            StmtKind::Block{ stmts: statements } => self.evaluate_block(stmt.span, statements),
            StmtKind::While { cond, body } => self.evaluate_while(cond, body),
            StmtKind::If { cond, then, else_branch } => {
                self.evaluate_if(stmt.span, cond, then, else_branch)
            },
            StmtKind::FuncDeclaration { name , params , return_type , body  } => {
                self.evaluate_function_declaration(name, params, return_type, body)
            },
            StmtKind::Return { value } => self.evaluate_return(value),
            StmtKind::Break => {
                self.break_hit = true;
                Ok(TrackedValue::from(Value::Nil))
            },
            StmtKind::Continue => {
                self.continue_hit = true;
                Ok(TrackedValue::from(Value::Nil))
            },
        }
    }

    fn evaluate_return(&mut self, value: &Expr) -> Result<TrackedValue, ()> {
        let return_value = self.evaluate_expression(value);
        self.return_hit = true;
        Ok(TrackedValue {
            value: return_value?.value,
            source: DataSource::Return,
        })
    }

    fn evaluate_function_declaration(
        &mut self, 
        name: &Symbol, 
        params: &Vec<(Symbol, Option<Type>)>, 
        return_type: &Option<Type>, 
        body: &Box<Stmt>
    ) -> Result<TrackedValue, ()> {
        // Add the parameters to the environment of the function body
        for (ident, _type_annotation) in params {
            // We use Nil as a placeholder value since we only care about the type here
            self.env.borrow_mut().define(*ident, TrackedValue {
                value: Value::Nil,
                source: DataSource::Variable(*ident),
            });
        }

        let value = TrackedValue::from(Value::Function(FunctionType::User { 
            name: name.clone(),
            params: params.clone(),
            return_type: // TODO: if no return type annotation, we should infer it after parsing the body
                return_type.clone().unwrap_or(Type::Any),
            body: body.clone(),
        }));

        self.env.borrow_mut().define(
            *name, 
            value.clone()
        );

        Ok(value)
    }

    fn evaluate_while(
        &mut self, 
        cond: &Box<Expr>, 
        body: &Box<Stmt>
    ) -> Result<TrackedValue, ()> {
        loop {
            if self.break_hit {
                self.break_hit = false;
                break;
            }
            if self.continue_hit {
                self.continue_hit = false;
            }

            let cond_value = self.evaluate_expression(cond)?;

            let condition = match cond_value.value {
                Value::Boolean(b) => b,
                _ => {
                    self.error(
                        cond.span, 
                        "while condition must evaluate to a boolean"
                    );
                    return Err(());
                }
            };

            if condition {
                self.execute_statement(body)?;
            } else {
                break;
            }
        }

        Ok(TrackedValue::from(Value::Nil))
    }

    fn evaluate_if(
        &mut self, 
        span: Span,
        cond: &Box<Expr>, 
        then_branch: &Box<Stmt>, 
        else_branch: &Option<Box<Stmt>>
    ) -> Result<TrackedValue, ()> {
        let cond_value = self.evaluate_expression(cond)?;

        let condition = match cond_value.value {
            Value::Boolean(b) => b,
            _ => {
                self.error(
                    cond.span, 
                    "if condition must evaluate to a boolean"
                );
                return Err(());
            }
        };

        self.emit(TraceEvent::BranchEnter { 
            statement_id: span.start, // using the start of the span as a unique ID for the branch statement
            condition_result: condition
        });

        let output = if condition {
            self.execute_statement(then_branch)
        } else {
            // TODO: handle else-if branches
            if let Some(else_branch) = else_branch {
                self.execute_statement(else_branch)
            } else {
                Ok(TrackedValue::from(Value::Nil))
            }
        };

        self.emit(TraceEvent::BranchExit { statement_id: span.start });

        output
    }

    fn evaluate_block(&mut self, span: Span, statements: &Vec<Stmt>) -> Result<TrackedValue, ()>{
        // Save the current scope
        let previous = self.env.clone();

        // Create a nested scope
        self.env = Environment::extend(previous.clone());

        self.emit(TraceEvent::ScopeEnter { scope_id: span.start }); // using the start of the span as a unique ID for the scope

        // Execute the statements
        let mut last_value = TrackedValue::from(Value::Nil);
        for stmt in statements {
            last_value = self.execute_statement(stmt)?;

            if self.break_hit || self.return_hit || self.continue_hit {
                break;
            }
        }

        // Pop the scope
        self.env = previous;

        self.emit(TraceEvent::ScopeExited { scope_id: span.start });

        Ok(last_value)
    }    

    fn evaluate_expression(&mut self, expr: &crate::parser::Expr) -> Result<TrackedValue, ()> {
        let span = expr.span.clone();
        match &expr.node {
            ExprKind::Literal(lit) => Ok(TrackedValue::from(lit.clone())),
            ExprKind::Binary { left, op, right } => {
                self.execute_binary_operation(left, op, right)
            },
            ExprKind::Unary { op, expr } => {
                self.evaluate_unary(op, expr)
            },
            ExprKind::Identifier(name) => {
                // Check the environment for the variable
                // TODO: this problem should be resolved by the resolver pass
                if let Some(value) = self.env.borrow().get(name) {
                    Ok(value)
                } else {
                    self.error(
                        span, 
                        format!("undefined variable '{}' found", self.resolve_symbol(*name))
                    );
                    Err(())
                }
            },
            ExprKind::Assign { name, value } => {
                self.execute_variable_assignment_statement(name, value, span)
            },
            ExprKind::ArrayLiteral(elements) => {
                self.execute_array_literal(elements)
            },
            ExprKind::Get { object, index } => {
                self.execute_array_get(object, index)
            },
            ExprKind::Set { object, index, value } => {
                self.execute_array_set(object, index, value)
            },
            ExprKind::FunctionCall { callee, arguments } => {
                self.function_call(callee, arguments)
            }
        }
    }

    fn evaluate_unary(&mut self, op: &TokenKind, expr: &Box<crate::common::span::Spanned<ExprKind>>) -> Result<TrackedValue, ()> {
        let tracked_value = self.evaluate_expression(expr)?;
        let calculated_value = match op {
            TokenKind::Minus => {
                match &tracked_value.value {
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
                match &tracked_value.value {
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
    
        Ok(TrackedValue::from(calculated_value))
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


        if *op == TokenKind::LT || *op == TokenKind::LEQ || *op == TokenKind::GT || *op == TokenKind::GEQ || *op == TokenKind::EQ || *op == TokenKind::NEQ {
            self.emit(TraceEvent::Compare { 
                left: left_val.source.clone(), 
                right: right_val.source.clone(), 
                operator: format!("{}", &op),
                result: matches!(result_value, Value::Boolean(true)),
            });
        }

        Ok(TrackedValue::from(result_value))
    }

    fn execute_let_statement(
        &mut self, 
        name: &Symbol, 
        _type_annotation: &Option<crate::common::Type>, 
        initializer: &Box<Expr>
    ) -> Result<TrackedValue, ()> {
        // Evaluate the initializer expression
        let value = self.evaluate_expression(initializer)?;

        // // If we have a type annotation, check that the value matches the type
        // if let Some(expected_type) = type_annotation {
        //     let value_type = value.get_type();
        //     if &value_type != expected_type {
        //         self.error(
        //             // TODO: change the span to incorporate the assignment aswell
        //             initializer.span, 
        //             format!(
        //                 "Variable was declared with type '{}' but expression has type '{}'",
        //                 expected_type,
        //                 value_type
        //             ),
        //         );
        //         return Err(());
        //     }
        // }

        // Define the destination identity
        let destination = DataSource::Variable(*name);
        self.emit(TraceEvent::Init {
            location: destination.clone(),
            value: value.clone(),
        });

        // Update the source of the value to be the variable itself
        let tracked_for_env = TrackedValue {
            value: value.value.clone(),
            source: destination,
        };

        self.env.borrow_mut().define(name.clone(), tracked_for_env.clone());
        Ok(value)
    }

    fn execute_variable_assignment_statement(
        &mut self, 
        name: &Symbol, 
        value: &Box<Expr>,
        span: Span,
    ) -> Result<TrackedValue, ()> {
        // Evaluate the expression
        let value = self.evaluate_expression(value)?;

        // Define the destination
        let destination = DataSource::Variable(*name);

        // Emit the trace event
        self.emit(TraceEvent::Assign {
            from: value.source.clone(),
            to: destination.clone(),
            value: value.clone(),
        });

        // Since the value now lives in the variable, we update its source
        // so that the next time it is moved, it reports this variable as its origin
        let updated_val = TrackedValue {
            value: value.value.clone(),
            source: destination,
        };

        if self.env.borrow_mut().assign(name, updated_val.clone()) {
            Ok(updated_val)
        } else {
            // TODO: analyser ensures this doesn't happen
            self.error(
                span, 
                format!(
                    "undefined variable '{}' found", 
                    self.resolve_symbol(*name)
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

            // implement the trait!!!
            self.emit(TraceEvent::Assign { 
                from: value.source.clone(), 
                to: target_source.clone(), 
                value: value.clone() 
            });

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

                if !self.env.borrow_mut().assign(&symbol, update_array_tracked) {
                    self.error(
                        obj_expr.span, 
                        format!(
                            "undefined variable '{}' found", 
                            self.resolve_symbol(symbol)
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
    
    fn function_call(&mut self, callee: &Expr, arguments: &[Expr]) -> Result<TrackedValue, ()> {
        if let Expr { node: ExprKind::Identifier(func_name), span: _ } = callee {
            let func = self.env.borrow().get(func_name);

            if let Some(TrackedValue { value: Value::Function(func_type), source: _ }) = func {
                match func_type {
                    FunctionType::Native { name: _, args: _, return_type: _,func } => {
                        // Evaluate arguments
                        let mut arg_values = Vec::new();
                        for arg_expr in arguments {
                            let arg_value = self.evaluate_expression(arg_expr)?;
                            arg_values.push(arg_value);
                        }

                        // Call the native function
                        match func(arg_values) {
                            Ok(result) => Ok(result),
                            Err(err_message) => {
                                self.error(
                                    callee.span, 
                                    format!("Error in native function call: {}", err_message)
                                );
                                Err(())
                            }
                        }
                    },
                    FunctionType::User { name , return_type: _, params: _, body: _ } => {
                        // we need to enter a new environment,
                        // define the parameters in that environment with the argument values,
                        // execute the body in that environment, and return the result

                        // Get the function declaration based on the name
                        let declaration_opt = self.env.borrow().get(&name);
                        let declaration = if let Some(TrackedValue { value: Value::Function(func_type), source: _ }) = declaration_opt {
                            if let FunctionType::User { name, return_type, params, body } =  func_type {
                                (name, return_type, params, body)
                            } else {
                                self.error(
                                    callee.span,
                                    format!("'{}' is not a function", self.resolve_symbol(name))
                                );
                                return Err(());
                            }
                        } else {
                            self.error(
                                callee.span,
                                format!("undefined function '{}' called", self.resolve_symbol(name))
                            );
                            return Err(());
                        };


                        let (_name, _return_type, params, body) = declaration;

                        // Save the current scope
                        let previous = self.env.clone();

                        // Create a nested scope
                        self.env = Environment::extend(previous.clone());

                        // Assign the parameters to the environment
                        for (i, param) in params.iter().enumerate() {
                            let (param_name, param_type) = param;

                            let arg_expr = &arguments[i];
                            let arg_value = self.evaluate_expression(arg_expr)?;

                            // Check type compatibility
                            if let Some(expected_type) = param_type {
                                if arg_value.get_type() != *expected_type && *expected_type != Type::Any {
                                    self.error(
                                        arg_expr.span, 
                                        format!(
                                            "Type mismatch for parameter '{}': expected '{:?}', found '{:?}'",
                                            self.resolve_symbol(*param_name),
                                            expected_type,
                                            arg_value.get_type()
                                        )
                                    );
                                    return Err(());
                                }
                            }

                            self.env.borrow_mut().define(
                                *param_name, 
                                TrackedValue {
                                    value: arg_value.value.clone(),
                                    source: DataSource::Variable(param_name.clone()),
                                }
                            );
                        }

                        // Execute the body
                        let last_value = self.execute_statement(&body)?;

                        if self.return_hit {
                            self.return_hit = false;
                            // we can return the value immediately without popping the scope
                            return Ok(last_value);
                        }

                        // Pop the scope
                        self.env = previous;

                        Ok(last_value)
                    },
                }
            } else {
                self.error(
                    callee.span,
                    format!(
                        "undefined function '{}' called", 
                        self.ctx.pool.borrow().resolve(*func_name)
                    )
                );
                Err(())
            }
        } else {
            self.error(
                callee.span, 
                "function call requires a function identifier as callee"
            );
            Err(())
        }
    }
}
