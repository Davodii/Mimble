mod ast;

#[cfg(test)]
mod tests;

use crate::common::{DiagnosticsSink, Span, StringPool, Type};
use crate::lexer::{Token, TokenKind};

pub use ast::{Stmt, Expr, LiteralValue, ExprKind, StmtKind};
pub struct Parser<'a>{
    tokens: Vec<Token>,
    pool: &'a mut StringPool,
    sink: &'a mut DiagnosticsSink,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(
        tokens: Vec<Token>, 
        pool: &'a mut StringPool,
        sink: &'a mut DiagnosticsSink) -> Self {
        Self { 
            tokens, 
            pool, 
            sink, 
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
        self.sink.report(span, message, crate::common::Severity::Error);
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

    // ----- Expression Parsing -----
    fn expression(&mut self) -> Result<Expr, ()> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ()> {
        let expr = self.or()?;

        if self.matches(TokenKind::Assign) {
            let value = self.assignment()?;
            let combined_span = expr.span.merge(value.span);

            match expr.node {
                ExprKind::Identifier(name) => {
                    return Ok(Expr {
                        node: ExprKind::Assign { 
                            name: name, 
                            value: Box::new(value) 
                        },
                        span: combined_span,
                    });
                },
                ExprKind::Get { object, index } => {
                    let combined_span = expr.span.merge(value.span);
                    return Ok(Expr {
                        node: ExprKind::Set { 
                            object, 
                            index, 
                            value: Box::new(value) 
                        },
                        span: combined_span,
                    });
                },
                _ => {
                    self.error(
                        expr.span,
                        // TODO: show the actual expression as well
                        format!("{} is not an assignable expression",expr.node.type_to_string()),
                    );
                    return Err(());
                }
            }
        }

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

        while self.matches_multiple(types) {
            let op = self.previous().kind.clone();
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
                break;
            }
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, ()> {
        let tok = self.advance().clone();
        let mut span = tok.span;

        let kind = match tok.kind {
            TokenKind::IntegerLiteral(val) => ExprKind::Literal(LiteralValue::Integer(val)),
            TokenKind::FloatLiteral(val) => ExprKind::Literal(LiteralValue::Float(val)),
            TokenKind::StringLiteral(val) => ExprKind::Literal(LiteralValue::String(self.pool.resolve(val).to_string())),
            TokenKind::True => ExprKind::Literal(LiteralValue::Boolean(true)),
            TokenKind::False => ExprKind::Literal(LiteralValue::Boolean(false)),
            TokenKind::Identifier(val) => ExprKind::Identifier(val),
            TokenKind::LeftParen => {
                let expr = self.expression()?;

                // Check for closing parentheses
                self.consume(TokenKind::RightParen, "Expected ')' to close set of parentheses")?;

                // For grouped expressions, we return the inner expression directly.
                // Note: Some people prefer to wrap this in a new Span that 
                // covers from '(' to ')', but returning the inner expr is fine.
                return Ok(expr);
            },
            TokenKind::LeftSquareBracket => {
                let mut elements: Vec<Expr> = Vec::new();

                if !self.check(TokenKind::RightSquareBracket) {
                    loop {
                        let element = self.expression()?;
                        elements.push(element);

                        if !self.matches(TokenKind::Comma) {
                            break;
                        }
                    }
                }

                self.consume(TokenKind::RightSquareBracket, "Expected ']' to close array literal")?;

                ExprKind::ArrayLiteral(elements)
            },
            _ => {
                self.error(
                    span, 
                    format!("Expected a valid expression but found '{}'", tok)
                );
                return Err(());
            }
        };

        // Update the span to include the entire atom
        span = span.merge(self.previous().span);

        Ok(Expr {
            node: kind,
            span,
        })
    }

    // ----- Statement Parsing -----
    fn statement(&mut self) -> Result<Stmt, ()> {
        if self.matches(TokenKind::If) {
            self.if_stmt()
        } else if self.matches(TokenKind::While) {
            self.while_stmt()
        } else if self.matches(TokenKind::Do) {
            todo!()
        } else if self.matches(TokenKind::Let) {
            self.declaration()
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

    fn if_stmt(&mut self) -> Result<Stmt, ()> {
        todo!()
    }

    fn while_stmt(&mut self) -> Result<Stmt, ()> {
        todo!()
    }

    fn declaration(&mut self) -> Result<Stmt, ()> {
        let name_token = self.consume_identifier("Expected an identifier after 'let'")?;
        let mut span = name_token.span;

        // Check if we have an optional type annotation
        let type_annotation = if self.matches(TokenKind::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.consume(TokenKind::Assign, "Expected '=' after in a 'let' statement")?;

        let initializer = self.expression()?;

        // TODO: this is so bad, we already know name_token is a TokenKind::Identifier
        let symbol = if let TokenKind::Identifier(sym) = name_token.kind.clone() {
            sym
        } else {
            self.error(
                name_token.span,
                "Expected identifier token",
            );
            return Err(());
        };

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
            Ok(Type::Array(Box::new(inner_type)))
        } else {
            match self.advance().kind {
                TokenKind::Integer => Ok(Type::Integer),
                TokenKind::Float => Ok(Type::Float),
                TokenKind::Boolean => Ok(Type::Boolean),
                TokenKind::String => Ok(Type::String),
                _ => {
                    self.error(self.previous().span, "Expected a type ('int', 'float', 'bool', 'string', 'array') after type definition (':')");
                    Err(())
                },
            }
        }
    }
}
