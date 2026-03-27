// TODO: implement a semantic analyser that checks for type errors, undefined variables, etc.

use std::collections::HashMap;

use crate::{
    common::{Severity, Symbol, Type, context::Context}, evaluator::environment::Environment, lexer::TokenKind, parser::{Expr, Stmt, StmtKind}
};

// - [ ] Undefined variables
// - [ ] Type errors (e.g. adding a number to a string)
// - [ ] Function call errors (e.g. wrong number of arguments, wrong argument types)
// - [ ] Control flow errors (e.g. break/continue outside of loops, return outside of functions)
// - [ ] Unreachable code

pub struct Analyser {
    ctx: Context,
    scopes: Vec<HashMap<Symbol, Type>>,
}

impl Analyser {
    pub fn new(ctx: Context) -> Self {
        Self { 
            ctx,
            scopes: Vec::new()
        }
    }

    pub fn seed_from_environment(&mut self, env: &Environment) {
        // Seed the analyser's scopes with the variables and types from the given environment
        self.enter_scope();
        
        // TODO: assuming only one global environment is passed with all globals defined in it
        for (symbol, ty) in &env.types {
            self.declare_variable(symbol.clone(), ty.clone()).unwrap(); // TODO: handle errors properly
        }
    }

    pub fn analyse(&mut self, code: &Vec<Stmt>) -> Result<(), ()> {
        // Enter a starting scope
        self.enter_scope();

        for stmt in code {
            self.analyse_stmt(&stmt)?;
        }

        Ok(())
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare_variable(&mut self, name: Symbol, ty: Type) -> Result<(), ()> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name) {
                // Variable already declared in this scope
                return Err(());
            }
            scope.insert(name, ty);
            Ok(())
        } else {
            // No scope to declare variable in
            Err(())
        }
    }

    fn lookup_variable(&self, name: &Symbol) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        None
    }

    fn analyse_stmt(&mut self, stmt: &Stmt) -> Result<(), ()> {
        let kind = &stmt.node;
        match kind {
            StmtKind::ExprStmt(spanned) => self.analyse_expression_stmt(spanned),
            StmtKind::If { cond, then, else_branch } => self.analyse_if_stmt(cond, then, else_branch),
            StmtKind::While { cond, body } => self.analyse_while_stmt(cond, body),
            StmtKind::Block { stmts } => self.analyse_block_stmt(stmts),
            StmtKind::LetStmt { name, type_annotation, initializer } => self.analyse_let_stmt(name, type_annotation, initializer),
            StmtKind::FuncDeclaration { name, params, return_type, body } => self.analyse_func_declaration(name, params, return_type, body),
        }
    }

    fn analyse_expression(&mut self, expr: &Expr) -> Result<Type, ()> {
        // Analyse the expression and return its type
        let kind = &expr.node;
        match kind {
            crate::parser::ExprKind::Binary { left, op, right } => self.analyse_binary(left, op, right),
            crate::parser::ExprKind::Unary { op, expr } => self.analyse_unary(op, expr),
            crate::parser::ExprKind::Literal(literal_value) => {
                // Get the type of the literal value
                Ok(literal_value.get_type())
            },
            crate::parser::ExprKind::Identifier(symbol) => {
                // Look up the variable in the current scope and return its type
                if let Some(ty) = self.lookup_variable(&symbol) {
                    Ok(ty)
                } else {
                    // Variable not found
                    self.ctx.diagnostics.borrow_mut().report(
                        expr.span, 
                        format!("Undefined variable: {}", symbol), // TODO: convert symbol to string
                        Severity::Error);
                    Err(())
                }
            },
            crate::parser::ExprKind::Assign { name, value } => self.analyse_assign(expr, name, value),
            crate::parser::ExprKind::ArrayLiteral(spanneds) => self.analyse_array_literal(spanneds),
            crate::parser::ExprKind::Get { object, index } => self.analyse_array_get(object, index),
            crate::parser::ExprKind::Set { object, index, value } => self.analyse_array_set(object, index, value),
            crate::parser::ExprKind::FunctionCall { callee, arguments } => self.analyse_function_call(callee, arguments),
        }
    }

    fn analyse_array_literal(&mut self, spanneds: &Vec<Expr>) -> Result<Type, ()> {
        // Analyse each element and check that they all have the same type
        let mut element_type = None;
        for spanned in spanneds {
            let ty = self.analyse_expression(spanned)?;
            if let Some(ref element_type) = element_type {
                if ty != *element_type {
                    self.ctx.diagnostics.borrow_mut().report(
                        spanned.span,
                        format!("Type error: array elements must all have the same type, found {} and {}", ty, element_type), // TODO: convert types to strings
                        Severity::Error);
                    return Err(());
                }
            } else {
                element_type = Some(ty);
            }
        }
        Ok(Type::Array(Box::new(element_type.unwrap_or(Type::Nil))))
    }
    
    fn analyse_assign(&mut self, expr: &Expr, name: &Symbol, value: &Expr) -> Result<Type, ()> {
        // Look up the variable in the current scope and check that it exists
        if let Some(var_type) = self.lookup_variable(&name) {
            // Analyse the value expression and get its type
            let value_type = self.analyse_expression(value)?;
    
            // Check that the value type matches the variable type
            if value_type == var_type {
                Ok(var_type)
            } else {
                self.ctx.diagnostics.borrow_mut().report(
                    expr.span, 
                    format!("Type error: cannot assign {} to variable of type {}", value_type, var_type), // TODO: convert types to strings
                    Severity::Error);
                Err(())
            }
        } else {
            // Variable not found
            self.ctx.diagnostics.borrow_mut().report(
                expr.span, 
                format!("Undefined variable: {}", name), // TODO: convert symbol to string
                Severity::Error);
            Err(())
        }
    }

    fn analyse_array_get(&mut self, array: &Expr, index: &Expr) -> Result<Type, ()> {
        // Analyse the index expression and check that it's an integer
        let index_type = self.analyse_expression(index)?;
        if index_type != Type::Integer {
            self.ctx.diagnostics.borrow_mut().report(
                index.span,
                format!("Type error: array index must be an integer, found {}", index_type), // TODO: convert types to strings
                Severity::Error);
            return Err(());
        }

        // Analyse the array expression and check that it's an array type
        let array_type = self.analyse_expression(array)?;
        if let Type::Array(element_type) = array_type {
            Ok((*element_type).clone())
        } else {
            self.ctx.diagnostics.borrow_mut().report(
                array.span,
                format!("Type error: expected array type, found {}", array_type), //
                Severity::Error);
            Err(())
        }
    }

    fn analyse_array_set(&mut self, array: &Expr, index: &Expr, value: &Expr) -> Result<Type, ()> {
        // Analyse the index expression and check that it's an integer
        let index_type = self.analyse_expression(index)?;
        if index_type != Type::Integer {
            self.ctx.diagnostics.borrow_mut().report(
                index.span,
                format!("Type error: array index must be an integer, found {}", index_type), // TODO: convert types to strings
                Severity::Error);
            return Err(());
        }

        // Analyse the array expression and check that it's an array type
        let array_type = self.analyse_expression(array)?;
        if let Type::Array(element_type) = array_type {
            // Analyse the value expression and check that it matches the element type
            let value_type = self.analyse_expression(value)?;
            if value_type == *element_type || (value_type == Type::Nil && *element_type != Type::Nil) {
                Ok(value_type)
            } else {
                self.ctx.diagnostics.borrow_mut().report(
                    value.span,
                    format!("Type error: expected array element type {}, found {}", element_type, value_type), // TODO: convert types to strings
                    Severity::Error);
                Err(())
            }
        } else {
            self.ctx.diagnostics.borrow_mut().report(
                array.span,
                format!("Type error: expected array type, found {}", array_type), //
                Severity::Error);
            Err(())
        }
    }

    fn analyse_function_call(&mut self, callee: &Expr, arguments: &Vec<Expr>) -> Result<Type, ()> {
        // Analyse the callee expression and check that it's a function type
        let callee_type = self.analyse_expression(callee)?;
        let (param_types, return_type) = if let Type::Function { param_types, return_type } = callee_type {
            (param_types, return_type)
        } else {
            self.ctx.diagnostics.borrow_mut().report(
                callee.span,
                format!("Type error: expected function type, found {}", callee_type), // TODO: convert types to strings
                Severity::Error);
            return Err(());
        };

        // Analyse each argument and check that it matches the corresponding parameter type
        if arguments.len() != param_types.len() {
            self.ctx.diagnostics.borrow_mut().report(
                callee.span,
                format!("Type error: expected {} arguments, found {}", param_types.len(), arguments.len()),
                Severity::Error);
            return Err(());
        }
        for (arg, param_type) in arguments.iter().zip(param_types.iter()) {
            let arg_type = self.analyse_expression(arg)?;
            if arg_type != *param_type {
                self.ctx.diagnostics.borrow_mut().report(
                    arg.span,
                    format!("Type error: expected argument of type {}, found {}", param_type, arg_type),
                    Severity::Error);
                return Err(());
            }
        }
    
        // Return the function's return type
        Ok(*return_type.clone())
    }

    fn analyse_binary(&mut self, left: &Expr, op: &TokenKind, right: &Expr) -> Result<Type, ()> {
        let left_type = self.analyse_expression(left)?;
        let right_type = self.analyse_expression(right)?;

        match op {
            TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash => {
                if (left_type == Type::Integer || left_type == Type::Float) && (right_type == Type::Integer || right_type == Type::Float) {
                    // If either operand is a float, the result is a float
                    if left_type == Type::Float || right_type == Type::Float {
                        Ok(Type::Float)
                    } else {
                        Ok(Type::Integer)
                    }
                } else {
                    self.ctx.diagnostics.borrow_mut().report(
                        left.span.merge(right.span),
                        format!("Type error: operator {:?} not supported for types {} and {}", op, left_type, right_type), // TODO: convert operator and types to strings
                        Severity::Error);
                    Err(())
                }
            },
            TokenKind::EQ | TokenKind::NEQ => {
                if left_type == right_type {
                    Ok(Type::Boolean)
                } else {
                    self.ctx.diagnostics.borrow_mut().report(
                        left.span.merge(right.span),
                        format!("Type error: operator {:?} not supported for types {} and {}", op, left_type, right_type), // TODO: convert operator and types to strings
                        Severity::Error);
                    Err(())
                }
            },
            TokenKind::LT | TokenKind::LEQ | TokenKind::GT | TokenKind::GEQ => {
                if (left_type == Type::Integer || left_type == Type::Float) && (right_type == Type::Integer || right_type == Type::Float) {
                    Ok(Type::Boolean)
                } else {
                    self.ctx.diagnostics.borrow_mut().report(
                        left.span.merge(right.span),
                        format!("Type error: operator {:?} not supported for types {} and {}", op, left_type, right_type), // TODO: convert operator and types to strings
                        Severity::Error);
                    Err(())
                }
            },
            TokenKind::And | TokenKind::Or => {
                if left_type == Type::Boolean && right_type == Type::Boolean {
                    Ok(Type::Boolean)
                } else {
                    self.ctx.diagnostics.borrow_mut().report(
                        left.span.merge(right.span),
                        format!("Type error: operator {:?} not supported for types {} and {}", op, left_type, right_type), // TODO: convert operator and types to strings
                        Severity::Error);
                    Err(())
                }
            },
            _ => {
                self.ctx.diagnostics.borrow_mut().report(
                    left.span.merge(right.span),
                    format!("Invalid binary operator: {:?}", op), // TODO: convert operator to string
                    Severity::Error);
                Err(())
            }
        }
    }

    fn analyse_unary(&mut self, op: &TokenKind, expr: &Expr) -> Result<Type, ()> {
        // Analyse the operand and get its type
        let operand_type = self.analyse_expression(expr)?;
        // Check that the operator is valid for the operand type and return the result type
        match op {
            crate::lexer::TokenKind::Plus | crate::lexer::TokenKind::Minus => {
                if operand_type == Type::Integer || operand_type == Type::Float {
                    Ok(operand_type)
                } else {
                    self.ctx.diagnostics.borrow_mut().report(
                        expr.span, 
                        format!("Type error: operator {:?} not supported for type {}", op, operand_type), // TODO: convert operator and type to strings
                        Severity::Error);
                    Err(())
                }
            },
            crate::lexer::TokenKind::Not => {
                if operand_type == Type::Boolean {
                    Ok(Type::Boolean)
                } else {
                    self.ctx.diagnostics.borrow_mut().report(
                        expr.span, 
                        format!("Type error: operator {:?} not supported for type {}", op, operand_type), // TODO: convert operator and type to strings
                        Severity::Error);
                    Err(())
                }
            },
            _ => {
                self.ctx.diagnostics.borrow_mut().report(
                    expr.span, 
                    format!("Invalid unary operator: {:?}", op), // TODO: convert operator to string
                    Severity::Error);
                Err(())
            }
        }
    }
    
    fn analyse_expression_stmt(&mut self, expr: &Expr) -> Result<(), ()> {
        // Analyse the expression and check for type errors
        self.analyse_expression(expr)?;
        Ok(())
    }

    fn analyse_if_stmt(&mut self, cond: &Expr, then: &Stmt, else_branch: &Option<Box<Stmt>>) -> Result<(), ()> {
        // Analyse the condition and check that it's a boolean
        let cond_type = self.analyse_expression(cond)?;
        if cond_type != Type::Boolean {
            self.ctx.diagnostics.borrow_mut().report(
                cond.span,
                format!("Type error: expected boolean condition, found {}", cond_type), // TODO: convert
                Severity::Error);
            return Err(());
        }

        // Analyse the then branch
        self.analyse_stmt(then)?;

        // Analyse the else branch if it exists
        if let Some(else_branch) = else_branch {
            self.analyse_stmt(else_branch)?;
        }

        Ok(())
    }

    fn analyse_while_stmt(&mut self, cond: &Expr, body: &Stmt) -> Result<(), ()> {
        // Analyse the condition and check that it's a boolean
        let cond_type = self.analyse_expression(cond)?;
        if cond_type != Type::Boolean {
            self.ctx.diagnostics.borrow_mut().report(
                cond.span,
                format!("Type error: expected boolean condition, found {}", cond_type), // TODO: convert
                Severity::Error);
            return Err(());
        }

        // Analyse the body
        self.analyse_stmt(body)?;

        // TODO: can check for unreachable code after a while loop with a constant true condition, but that requires some form of constant folding which we haven't implemented yet
        // TODO: also check for break/continue statements
        // TODO: we could also check for infinite loops by looking for while loops with a constant true condition and no break statements

        Ok(())
    }

    fn analyse_block_stmt(&mut self, stmts: &Vec<Stmt>) -> Result<(), ()> {
        // Enter a new scope
        self.enter_scope();

        for stmt in stmts {
            self.analyse_stmt(stmt)?;
        }

        // Exit the scope
        self.exit_scope();

        Ok(())
    }

    fn analyse_let_stmt(&mut self, name: &Symbol, type_annotation: &Option<Type>, initializer: &Expr) -> Result<(), ()> {
        // Analyse the initializer and get its type
        // If there's a type annotation, check that it matches the initializer's type
        // Declare the variable in the current scope
        let initializer_type = self.analyse_expression(initializer)?;
        if let Some(ref annotation) = *type_annotation {
            if annotation != &initializer_type {
                self.ctx.diagnostics.borrow_mut().report(
                    initializer.span,
                    format!("Type error: expected {}, found {}", annotation, initializer_type),
                    Severity::Error
                );
                return Err(());
            }
        }
        self.declare_variable(name.clone(), initializer_type)?;
        Ok(())
    }

    fn analyse_func_declaration(&mut self, name: &Symbol, params: &Vec<(Symbol, Option<Type>)>, return_type: &Option<Type>, body: &Stmt) -> Result<(), ()> {
        // Declare the function in the current scope with a placeholder type (e.g. "function")
        self.declare_variable(
            name.clone(), 
            Type::Function { 
                param_types: Vec::new(), 
                return_type: Box::new(Type::Nil) 
            }
        )?;

        // Analyse the function body in a new scope where the parameters are declared
        self.enter_scope();
        for (param_name, param_type) in params {
            // If the parameter has a type annotation, use it. Otherwise, use a placeholder type
            let ty = param_type.clone().unwrap_or(Type::Nil);
            self.declare_variable(param_name.clone(), ty)?;
        }

        self.analyse_stmt(body)?;
        self.exit_scope();

        // After analysing the body, update the function's type in the current scope with the correct parameter and return types
        // TODO: we need to get the actual parameter and return types from the body analysis, which is a bit tricky. For now, we'll just leave it as a placeholder.
        Ok(())
    }

}