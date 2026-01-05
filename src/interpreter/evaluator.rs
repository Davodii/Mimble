use super::environment::Environment;
use super::runtime_value::RuntimeValue;
use super::error::RuntimeError;
use crate::parser::{Stmt, StmtKind, Expr, ExprKind, LiteralValue};

use crate::common::{DiagnosticsSink, Span, Symbol, StringPool};
use crate::lexer::TokenKind;

pub struct WalkerEvaluator<'a> {
    // fields omitted
    environment: Environment,
    pool: &'a mut StringPool,
    sink: &'a mut DiagnosticsSink,
}

impl<'a> WalkerEvaluator<'a> {
    pub fn new(pool: &'a mut StringPool, sink: &'a mut DiagnosticsSink) -> Self {
        Self {
            environment: Environment::new(),
            pool,
            sink,
        }
    }

    pub fn interpret(&mut self, code: Vec<Stmt>) -> Result<RuntimeValue, RuntimeError> {
        let mut value = RuntimeValue::Nil;
        
        for stmt in code {
            value = self.execute_statement(&stmt)?;
        }

        Ok(value)
    }

    fn execute_statement(&mut self, stmt: &Stmt) -> Result<RuntimeValue, RuntimeError> {
        match stmt {
            Stmt{node: StmtKind::ExprStmt(expr), span:_} => {
                self.evaluate_expression(&expr)
            },
            Stmt{node: StmtKind::VarDeclaration { name, var_type: _, initializer }, span:_} => {
                let value = if let Some(init_expr) = initializer {
                    self.evaluate_expression(init_expr)?
                } else {
                    RuntimeValue::Nil
                };

                let TokenKind::Identifier(ref sym) = name.kind else {
                    return Err(RuntimeError::TypeMismatch { expected: "identifier".to_string(), found: "other".to_string() });
                };

                self.environment.assign(sym, value.clone());
                Ok(value)
            }
            _ => todo!(),
        }
    }

    fn evaluate_expression(&mut self, expr: &crate::parser::Expr) -> Result<RuntimeValue, RuntimeError> {
        match expr {
            Expr{node: ExprKind::Literal(lit), span: _} => {
                match lit {
                    LiteralValue::Number(n) => Ok(RuntimeValue::Number(*n)),
                    LiteralValue::String(s) => {
                        // TODO: optimize string handling by defering to_string() until necessary (use Symbols internally)
                        let string_value = self.pool.resolve(*s).to_string();
                        Ok(RuntimeValue::String(string_value))
                    },
                    LiteralValue::Boolean(b) => Ok(RuntimeValue::Boolean(*b)),
                    LiteralValue::Nil => Ok(RuntimeValue::Nil),
                    LiteralValue::Error => todo!(),
                }
            },
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
                    _ => Err(RuntimeError::TypeMismatch { expected: "TODO".to_string(), found: "other".to_string() }),
                }
            },
            Expr{node: ExprKind::Unary { op, expr }, span: _} => {
                let value = self.evaluate_expression(expr)?;
                match op {
                    TokenKind::Minus => value.negate(),
                    TokenKind::Not => value.not(),
                    _ => Err(RuntimeError::TypeMismatch { expected: "TODO".to_string(), found: "other".to_string() }),
                }
            },
            Expr{node: ExprKind::Identifier(name), span: _} => {
                if let Some(value) = self.environment.get(name) {
                    Ok(value)
                } else {
                    todo!("Undefined variable error handling")
                    // Err(RuntimeError::UndefinedVariable(name.clone()))
                }
            },
            Expr{node: ExprKind::Assign { name: name_expr, value }, span: _} => {
                let val = self.evaluate_expression(value)?;

                // Convert name into an identifier
                let var_name = if let Expr{node: ExprKind::Identifier(symbol), span: _} = &**name_expr {
                    symbol
                } else {
                    todo!("Assignment to non-identifier error handling")
                };

                if self.environment.assign(var_name, val.clone()) {
                    Ok(val)
                } else {
                    todo!("Undefined variable error handling on assignment")
                }
            },
            _ => todo!(),
        }
    }
}