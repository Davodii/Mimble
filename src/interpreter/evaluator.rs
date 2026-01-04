use crate::parser::Program;
use super::environment::Environment;
use super::runtime_value::RuntimeValue;
use super::error::RuntimeError;

pub struct WalkerEvaluator {
    // fields omitted
    environment: Environment,
}

impl WalkerEvaluator {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, code: Program) -> Result<RuntimeValue, RuntimeError> {
        let mut value = RuntimeValue::Nil;
        
        for stmt in code.statements {
            value = self.execute_statement(&stmt)?;
        }

        Ok(value)
    }

    fn execute_statement(&mut self, stmt: &crate::parser::Stmt) -> Result<RuntimeValue, RuntimeError> {
        match stmt {
            crate::parser::Stmt::ExprStmt(expr) => {
                self.evaluate_expression(&expr)
            },
            crate::parser::Stmt::VarDeclaration { name, var_type, initializer } => {
                let value = if let Some(init_expr) = initializer {
                    self.evaluate_expression(init_expr)?
                } else {
                    RuntimeValue::Nil
                };

                self.environment.assign(&name.lexeme, value.clone());
                Ok(value)
            }
            _ => todo!(),
        }
    }

    fn evaluate_expression(&mut self, expr: &crate::parser::Expr) -> Result<RuntimeValue, RuntimeError> {
        match expr {
            crate::parser::Expr::Literal(lit) => {
                match lit {
                    crate::parser::LiteralValue::Number(n) => Ok(RuntimeValue::Number(*n)),
                    crate::parser::LiteralValue::String(s) => Ok(RuntimeValue::String(s.clone())),
                    crate::parser::LiteralValue::Boolean(b) => Ok(RuntimeValue::Boolean(*b)),
                    crate::parser::LiteralValue::Nil => Ok(RuntimeValue::Nil),
                    crate::parser::LiteralValue::Error => todo!(),
                }
            },
            crate::parser::Expr::Binary { left, op, right } => {
                // Evaluate the left and right expressions
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;

                match op {
                    crate::lexer::TokenKind::Plus => left_val.add(&right_val),
                    crate::lexer::TokenKind::Minus => left_val.subtract(&right_val),
                    crate::lexer::TokenKind::Star => left_val.multiply(&right_val),
                    crate::lexer::TokenKind::Slash => left_val.divide(&right_val),
                    crate::lexer::TokenKind::Modulus => left_val.modulus(&right_val),
                    crate::lexer::TokenKind::And => left_val.and(&right_val),
                    crate::lexer::TokenKind::Or => left_val.or(&right_val),
                    crate::lexer::TokenKind::EQ => left_val.equals(&right_val),
                    crate::lexer::TokenKind::NEQ => left_val.not_equals(&right_val),
                    crate::lexer::TokenKind::LT => left_val.less_than(&right_val),
                    crate::lexer::TokenKind::LEQ => left_val.less_than_equal(&right_val),
                    crate::lexer::TokenKind::GT => left_val.greater_than(&right_val),
                    crate::lexer::TokenKind::GEQ => left_val.greater_than_equal(&right_val),
                    _ => Err(RuntimeError::TypeMismatch { expected: "TODO".to_string(), found: "other".to_string() }),
                }
            },
            crate::parser::Expr::Unary { op, expr } => {
                let value = self.evaluate_expression(expr)?;
                match op {
                    crate::lexer::TokenKind::Minus => value.negate(),
                    crate::lexer::TokenKind::Not => value.not(),
                    _ => Err(RuntimeError::TypeMismatch { expected: "TODO".to_string(), found: "other".to_string() }),
                }
            },
            crate::parser::Expr::Identifier(token) => {
                let name = &token.lexeme;
                if let Some(value) = self.environment.get(name) {
                    Ok(value)
                } else {
                    Err(RuntimeError::UndefinedVariable(name.clone()))
                }
            },
            crate::parser::Expr::Assign { name, value } => {
                let val = self.evaluate_expression(value)?;

                // Convert name into an identifier
                let var_name = if let crate::parser::Expr::Identifier(token) = &**name {
                    &token.lexeme
                } else {
                    return Err(RuntimeError::TypeMismatch { expected: "identifier".to_string(), found: "other".to_string() });
                };

                if self.environment.assign(var_name, val.clone()) {
                    Ok(val)
                } else {
                    Err(RuntimeError::UndefinedVariable(var_name.clone()))
                }
            },
            crate::parser::Expr::Error => todo!(),
        }
    }
}