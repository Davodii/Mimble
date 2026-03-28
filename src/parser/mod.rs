mod ast;

#[cfg(test)]
mod tests;

use crate::common::context::Context;
use crate::common::{Span, Symbol, Type};
use crate::lexer::{Token, TokenKind};

pub use ast::{Stmt, Expr, LiteralValue, ExprKind, StmtKind};
pub struct Parser {
    tokens: Vec<Token>,
    ctx: Context,
    current: usize,
}

impl Parser {
    pub fn new(
        tokens: Vec<Token>, 
        ctx: Context
    ) -> Self {
        Self { 
            tokens, 
            ctx,
            current: 0 
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        if self.tokens.is_empty() || self.tokens.last().unwrap().kind != TokenKind::EOF {
            todo!("Parser Error: Expected EOF token at end of input");
        }

        let mut stmts: Vec<Stmt> = Vec::new();

        while !self.is_at_end(){
            match self.statement() {
                Ok(stmt) => stmts.push(stmt),
                Err(_) => {
                    // If we hit an error, try to synchronise and continue parsing
                    self.synchronise();
                }
            }
        }

        stmts
    }

    // ----- Utilities -----
    fn synchronise(&mut self) {
        self.advance(); // move past the error token

        while !self.is_at_end() {
            // TODO: check for new line

            match self.peek().kind {
                TokenKind::End | TokenKind::If | TokenKind::While | TokenKind::Let => return,
                _ => self.advance(),
            };
        }
    }

    fn error(&mut self, span: Span, message: impl Into<String>) -> (){
        self.ctx.diagnostics.borrow_mut().report(span, message, crate::common::Severity::Error);
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {        
            self.current += 1;
        }
        
        self.previous()
    }

    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<Token, ()> {
        if self.check(kind) {
            Ok(self.advance().clone())
        } else {
            let span = self.peek().span;
            self.error(span, message);
            Err(())
        }
    }

    fn consume_identifier(&mut self, message: &str) -> Result<Token, ()> {
        if let TokenKind::Identifier(_) = &self.peek().kind {
            self.advance();
            Ok(self.previous().clone())
        } else {
            let span = self.peek().span;
            self.error(span, message);
            Err(())
        }
    }

    fn matches(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn matches_multiple(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.matches(kind.clone()) {
                return true;
            }
        }
        false
    }

    fn check(&self, kind: TokenKind) -> bool {
        !self.is_at_end() && self.peek().kind == kind
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::EOF
    }

    fn make_expr(&mut self, node: ExprKind, span: Span) -> Result<Expr, ()> {
        Ok(Expr { node, span  })
    }

    // ----- Expression Parsing -----
    fn expression(&mut self) -> Result<Expr, ()> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ()> {
        let expr = self.or()?;

        if self.matches(TokenKind::Assign) {
            let value = self.assignment()?;
            let combined_span = expr.span.merge(value.span);

            let node = match expr.node {
                ExprKind::Identifier(name) => ExprKind::Assign { 
                    name: name, 
                    value: Box::new(value) 
                },
                ExprKind::Get { object, index } => ExprKind::Set { 
                    object, 
                    index, 
                    value: Box::new(value) 
                },
                _ => {
                    self.error(
                        expr.span,
                        format!("Invalid assignment target: {}",expr.node.kind_to_string()),
                    );
                    return Err(());
                }
            };

            return Ok(Expr { node, span: combined_span });
        }

        // Base case, no assignment
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ()> {
        self.binary_op(&[TokenKind::Or], Parser::and)
    }

    fn and(&mut self) -> Result<Expr, ()> {
        self.binary_op(&[TokenKind::And], Parser::not)
    }

    fn not(&mut self) -> Result<Expr, ()> {
        if self.matches(TokenKind::Not) {
            let expr = self.not()?;

            let combined_span = expr.span.merge(expr.span);
            return Ok(Expr {
                node: ExprKind::Unary { 
                    op: TokenKind::Not, 
                    expr: Box::new(expr) 
                },
                span: combined_span,

            });
        }

        self.comparison()
    }

    fn comparison(&mut self) -> Result<Expr, ()> {
        self.binary_op(
            &[
                TokenKind::LT, TokenKind::LEQ,
                TokenKind::GT, TokenKind::GEQ,
                TokenKind::EQ, TokenKind::NEQ,
            ],
            Parser::arithmetic
        )
    }

    fn arithmetic(&mut self) -> Result<Expr, ()> {
        self.binary_op(
            &[TokenKind::Plus, TokenKind::Minus],
            Parser::term
        )
    }

    fn term(&mut self) -> Result<Expr, ()> {
        self.binary_op(
            &[TokenKind::Star, TokenKind::Slash, TokenKind::Modulus],
            Parser::factor
        )
    }

    fn binary_op<F>(&mut self, types: &[TokenKind], next: F) -> Result<Expr, ()> 
    where 
        F: Fn(&mut Self) -> Result<Expr, ()>,
    {
        // Call the "next" level 
        let mut expr = next(self)?;

        // Keep parsing the same level binary operations
        while self.matches_multiple(types) {
            let op = self.previous().kind.clone();

            // Parse the right-hand side
            let right = next(self)?;

            let combined_span = expr.span.merge(right.span);
            expr = Expr {
                node: ExprKind::Binary { 
                    left: Box::new(expr), 
                    op, 
                    right: Box::new(right)
                },
                span: combined_span,
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ()> {
        if self.matches_multiple(&[TokenKind::Plus, TokenKind::Minus]) {
            let span = self.previous().span;
            let op = self.previous().kind.clone();

            let right = self.factor()?;
            let span = right.span.merge(span);

            return Ok(
                Expr {
                    node: ExprKind::Unary { op, expr: Box::new(right) },
                    span,
                }
            );
        }

        self.postfix()
    }

    fn postfix(&mut self) -> Result<Expr, ()> {
        let mut expr = self.primary()?;
        loop {
            if self.matches(TokenKind::LeftSquareBracket) {
                // Array indexing
                let index = self.expression()?;

                let right_bracket =  self.consume(TokenKind::RightSquareBracket, "Expected ']' after array index expression")?;

                let combined_span = expr.span.merge(right_bracket.span);
                expr = Expr {
                    node: ExprKind::Get {
                        object: Box::new(expr),
                        index: Box::new(index),
                    },
                    span: combined_span,
                };
            } else {
                // TODO: check for calls
                break;
            }
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, ()> {
        let tok = self.advance().clone();
        let span = tok.span;

        match tok.kind {
            // Literals
            TokenKind::IntegerLiteral(val) => self.make_expr(ExprKind::Literal(LiteralValue::Integer(val)), span),
            TokenKind::FloatLiteral(val) => self.make_expr(ExprKind::Literal(LiteralValue::Float(val)), span),
            TokenKind::True => self.make_expr(ExprKind::Literal(LiteralValue::Boolean(true)), span),
            TokenKind::False => self.make_expr(ExprKind::Literal(LiteralValue::Boolean(false)), span),
            TokenKind::StringLiteral(val) => {
                self.make_expr(ExprKind::Literal(LiteralValue::String(val)), span)
            },
            // Identifiers and function calls
            TokenKind::Identifier(val) => {
                let expr = Expr { node: ExprKind::Identifier(val.clone()), span: span.clone() };

                if self.matches(TokenKind::LeftParen) {
                    self.finish_call(expr)
                } else {
                    Ok(expr)
                }
            },

            // Groupings
            TokenKind::LeftParen => self.grouping_expression(),
            TokenKind::LeftSquareBracket => self.array_literal(span),

            _ => {
                self.error(
                    span, 
                    format!("Expected a valid expression but found '{}'", tok)
                );
                return Err(());
            }
        }
    }

    fn grouping_expression(&mut self) -> Result<Expr, ()> {
        let expr = self.expression()?;
        self.consume(TokenKind::RightParen, "Expected ')' to close set of parentheses")?;
        Ok(expr)
    }

    fn array_literal(&mut self, open_span: Span) -> Result<Expr, ()> {
        let mut elements: Vec<Expr> = Vec::new();
        if !self.check(TokenKind::RightSquareBracket) {
            loop {
                elements.push(self.expression()?);
                if !self.matches(TokenKind::Comma) { break; }
            }
        }

        let end_span = self.consume(TokenKind::RightSquareBracket, "Expected ']' to close array literal")?.span;
        Ok(Expr {
            node: ExprKind::ArrayLiteral(elements),
            span: open_span.merge(end_span),
        })
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ()> {
        let mut arguments = Vec::new();
        if !self.check(TokenKind::RightParen) {
            loop {
                arguments.push(self.expression()?);
                if !self.matches(TokenKind::Comma) { break; }
            }
        }

        let r_paren = self.consume(TokenKind::RightParen, "Expected ')' after function arguments")?;

        Ok(Expr {
            span: callee.span.merge(r_paren.span),
            node: ExprKind::FunctionCall { callee: Box::new(callee), arguments },
        })
    }

    // ----- Statement Parsing -----
    fn statement(&mut self) -> Result<Stmt, ()> {
        if self.matches(TokenKind::If) {
            self.if_stmt()
        } else if self.matches(TokenKind::While) {
            self.while_stmt()
        } else if self.matches(TokenKind::Let) {
            self.declaration()
        } else if self.check(TokenKind::Do) {
            self.block()
        } else if self.matches(TokenKind::Func) {
            self.func_declaration()
        } else if self.matches(TokenKind::Return) {
            let value = self.expression()?;
            let span = self.previous().span.merge(value.span);
            Ok(Stmt {
                node: StmtKind::Return { value: Box::new(value) },
                span,
            })
        } else if self.matches(TokenKind::Break) {
            let span = self.previous().span;
            Ok(Stmt {
                node: StmtKind::Break,
                span,
            })
        } else if self.matches(TokenKind::Continue) {
            let span = self.previous().span;
            Ok(Stmt {
                node: StmtKind::Continue,
                span,
            })
        } else {
            // fallback: expression statement
            let expr = self.expression()?;
            let span = expr.span;
            Ok(Stmt {
                node: StmtKind::ExprStmt(Box::new(expr)),
                span: span,
            })
        }
    }

    fn func_declaration(&mut self) -> Result<Stmt, ()> {
        let name = Box::new(self.consume_identifier("Expected a function name.")?);
        let start = name.span.clone();

        let symbol = if let TokenKind::Identifier(sym) = name.kind {
            sym
        } else {
            return Err(());
        };

        self.consume(TokenKind::LeftParen, "Expected '(' after function name")?;

        // Parse the parameters
        let mut params: Vec<(Symbol, Option<Type>)> = Vec::new();
        if !self.check(TokenKind::RightParen) {
            loop {
                let param_name_token = self.consume_identifier("Expected parameter name")?;
                let param_symbol = if let TokenKind::Identifier(sym) = param_name_token.kind {
                    sym
                } else {
                    return Err(());
                };

                // Check for optional type annotation
                let param_type = if self.matches(TokenKind::Colon) {
                    Some(self.parse_type()?)
                } else {
                    None
                };

                params.push((param_symbol, param_type));

                if !self.matches(TokenKind::Comma) { break; }
            }
        }

        self.consume(TokenKind::RightParen, "Expected ')' after function parameters")?;

        // Optional return type annotation
        let return_type = if self.matches(TokenKind::Colon) {
            // Optional return type annotation
            Some(self.parse_type()?)
        } else {
            // TODO: if no return type annotation, we should infer it after parsing the body
            None
        };

        let body = Box::new(self.block()?);
        let end = start.merge(body.span);

        Ok(Stmt {
            node: StmtKind::FuncDeclaration { 
                name: symbol, 
                params: params, 
                return_type: return_type, 
                body: body 
            },
            span: end
        })
    }

    fn if_stmt(&mut self) -> Result<Stmt, ()> {
        let condition = Box::new(self.expression()?);
        let start = condition.span.clone();

        // Begin a new scope
        let then_branch = Box::new(self.block()?);

        // TODO: parse else if branches

        let else_branch = if self.matches(TokenKind::Else) {
            Some(Box::new(self.block()?))
        } else {
            None
        };

        Ok(Stmt {
            node: StmtKind::If {
                cond: condition,
                then: then_branch,
                else_branch,
            },
            span: start.merge(self.previous().span),
        })
    }

    fn while_stmt(&mut self) -> Result<Stmt, ()> {
        let condition = Box::new(self.expression()?);
        let start = condition.span.clone();

        // Begin a new scope
        let body = Box::new(self.block()?);

        Ok(Stmt {
            node: StmtKind::While {
                cond: condition,
                body,
            },
            span: start.merge(self.previous().span),
        })
    }

    fn block(&mut self) -> Result<Stmt, ()> {
        let start = self.consume(TokenKind::Do, "Expected 'do' to start a block")?;

        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(TokenKind::End) && !self.is_at_end() {
            let stmt = self.statement()?;
            statements.push(stmt);
        }

        let end = self.consume(TokenKind::End, "Expected 'end' to close a block")?;

        Ok(Stmt {
            node: StmtKind::Block{ stmts: statements },
            span: start.span.merge(end.span),
        })
    }

    fn declaration(&mut self) -> Result<Stmt, ()> {
        let name_token = self.consume_identifier("Expected an identifier after 'let'")?;

        let symbol = match name_token.kind {
            TokenKind::Identifier(sym) => sym,
            _ => unreachable!("consume_identifier guaranteed an identifier"),
        };

        // Check if we have an optional type annotation
        let type_annotation = if self.matches(TokenKind::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.consume(TokenKind::Assign, "Expected '=' after in a 'let' statement")?;
        let initializer = self.expression()?;

        let span = name_token.span.merge(initializer.span);

        Ok(Stmt {
            node: StmtKind::LetStmt {
                name: symbol,
                type_annotation,
                initializer: Box::new(initializer),
            },
            span,
        })
    }

    fn parse_type(&mut self) -> Result<Type, ()> {
        if self.matches(TokenKind::LeftSquareBracket) {
            let inner_type = self.parse_type()?;
            self.consume(TokenKind::RightSquareBracket, "Expected ']' after array type")?;
            return Ok(Type::Array(Box::new(inner_type)));
        }
        let tok = self.advance();
        match tok.kind {
            TokenKind::Integer  => Ok(Type::Integer),
            TokenKind::Float    => Ok(Type::Float),
            TokenKind::Boolean  => Ok(Type::Boolean),
            TokenKind::String   => Ok(Type::String),
            _ => {
                self.error(tok.span, "Expected a type (int, float, bool, string, or [type])");
                Err(())
            }
        }
    }
}
