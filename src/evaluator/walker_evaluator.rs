use super::environment::Environment;
use super::runtime_value::RuntimeValue;
use crate::parser::{Stmt, StmtKind, Expr, ExprKind};

use crate::common::{DiagnosticsSink, Span, StringPool, Symbol};
use crate::lexer::TokenKind;

pub struct WalkerEvaluator<'a> {
    // fields omitted
    environment: &'a mut Environment,
    pool: &'a StringPool,
    sink: &'a mut DiagnosticsSink,
}

impl<'a> WalkerEvaluator<'a> {
    pub fn new(env: &'a mut Environment, pool: &'a StringPool, sink: &'a mut DiagnosticsSink) -> Self {
        Self {
            environment: env,
            pool,
            sink,
        }
    }

    pub fn interpret(&mut self, code: Vec<Stmt>) -> Result<RuntimeValue, ()> {
        let mut value = RuntimeValue::Nil;
        
        for stmt in code {
            match self.execute_statement(&stmt) {
                Ok(val) => value = val,
                Err(err) => {
                    // Emit runtime error to diagnostics sink
                    self.sink.report(
                        stmt.span,
                        format!("{:?}", err), 
                        crate::common::Severity::Error, 
                    );
                    return Err(());
                }
            }
        }

        Ok(value)
    }

    fn error(&mut self, span: Span, message: impl Into<String>){
        self.sink.report(
            span, 
            message.into(),
            crate::common::Severity::Error,
        );
    }

    fn execute_statement(&mut self, stmt: &Stmt) -> Result<RuntimeValue, ()> {
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

    fn evaluate_expression(&mut self, expr: &crate::parser::Expr) -> Result<RuntimeValue, ()> {
        match expr {
            Expr{node: ExprKind::Literal(lit), span: _} => Ok(RuntimeValue::from(lit.clone())),
            Expr{node: ExprKind::Binary { left, op, right }, span: _} => {
                // Evaluate the left and right expressions
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;

                match op {
                    TokenKind::Plus => left_val.add(&right_val),
                    TokenKind::Minus => left_val.subtract(&right_val),
                    TokenKind::Star => left_val.multiply(&right_val),
                    TokenKind::Slash => left_val.divide(&right_val),
                    TokenKind::Modulus => left_val.modulus(&right_val),
                    TokenKind::And => left_val.and(&right_val),
                    TokenKind::Or => left_val.or(&right_val),
                    TokenKind::EQ => left_val.equals(&right_val),
                    TokenKind::NEQ => left_val.not_equals(&right_val),
                    TokenKind::LT => left_val.less_than(&right_val),
                    TokenKind::LEQ => left_val.less_than_equal(&right_val),
                    TokenKind::GT => left_val.greater_than(&right_val),
                    TokenKind::GEQ => left_val.greater_than_equal(&right_val),
                    _ => todo!("Handle type mismatches"),
                }
            },
            Expr{node: ExprKind::Unary { op, expr }, span: _} => {
                let value = self.evaluate_expression(expr)?;
                match op {
                    TokenKind::Minus => value.negate(),
                    TokenKind::Not => value.not(),
                    _ => todo!("Handle type mismatches"),
                }
            },
            Expr{node: ExprKind::Identifier(name), span: _} => {
                if let Some(value) = self.environment.get(name) {
                    Ok(value)
                } else {
                    todo!("Undefined variable error handling {}", self.pool.resolve(*name));
                    // Err(RuntimeError::UndefinedVariable(name.clone()))
                }
            },
            Expr{node: ExprKind::Assign { name, value }, span: _} => {
                let val = self.evaluate_expression(value)?;

                if self.environment.assign(name, val.clone()) {
                    Ok(val)
                } else {
                    todo!("Undefined variable error handling on assignment")
                }
            },
            Expr{node: ExprKind::ArrayLiteral(elements), span: _} => {
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
                Ok(RuntimeValue::Array{
                    elements: values,
                    element_type: None,
                })
            },
            Expr{node: ExprKind::Get { object, index }, span: _} => {
                let object_val = self.evaluate_expression(object)?;
                let index_val = self.evaluate_expression(index)?;

                match (object_val, index_val) {
                    (RuntimeValue::Array{elements: arr, element_type: _}, RuntimeValue::Integer(n)) => {
                        let idx = n as usize;
                        match arr.get(idx) {
                            Some(val) => Ok(val.clone()),
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
            },
            Expr{node: ExprKind::Set { object, index, value }, span: _} => {
                if let ExprKind::Identifier(name) = object.node {
                    let object_val = self.evaluate_expression(object)?;
                    let index_val = self.evaluate_expression(index)?;
                    let value_val = self.evaluate_expression(value)?;

                    match (object_val, index_val) {
                        (RuntimeValue::Array{elements: mut arr, element_type}, RuntimeValue::Integer(n)) => {
                            let idx = n as usize;

                            if idx < arr.len() {
                                // Check type compatibility
                                if let Some(elem_type) = element_type {
                                    if value_val.get_type() != elem_type {
                                        self.error(
                                            value.span, 
                                            format!(
                                                "Array only accepts elements of type '{:?}', but found '{:?}'",
                                                elem_type,
                                                value_val.get_type()
                                            )
                                        );
                                        return Err(());
                                    }
                                }

                                arr[idx] = value_val.clone();

                                let new_arr = RuntimeValue::Array{elements: arr, element_type: None};

                                if !self.environment.assign(&name, new_arr.clone()) {
                                    todo!("Undefined variable error handling on array set assignment");
                                }

                                Ok(value_val)
                            } else {
                                self.error(
                                    index.span, 
                                    "array index out of bounds"
                                );
                                Err(())
                            }
                        },
                        _ => {
                            self.error(
                                index.span, 
                                "expected array with an integer index"
                            );
                            Err(())
                        },
                    }
                } else {
                    todo!("Invalid assignment target error handling")
                }
            },
        }
    }

    fn execute_let_statement(
        &mut self, name: &Symbol, 
        type_annotation: &Option<crate::common::Type>, 
        initializer: &Box<Expr>) 
    -> Result<RuntimeValue, ()> {
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

        self.environment.define(name.clone(), value.clone());
        Ok(value)
    }
}
