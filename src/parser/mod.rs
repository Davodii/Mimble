mod ast;
mod error;

#[cfg(test)]
mod tests;

// pub use error::{ParserError, ParserErrorKind};

use crate::common::{StringPool, DiagnosticsSink};
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
                    todo!("Need to synchronise");
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
                TokenKind::End
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Let => {
                    return;
                },

                _ => {
                    self.advance();
                },
            }
        }
    }

    // fn error(&mut self, kind: ParserErrorKind) -> ParserError{
    //     let err = ParserError { 
    //         kind, 
    //         location: self.peek().loc.clone() 
    //     };

    //     self.sink.report(err.to_diagnostic());

    //     self.synchronise();

    //     return err;
    // }

    fn advance(&mut self) -> &Token {
        self.current += 1;
        self.previous()
    }

    fn matches(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else { false }
    }

    fn matches_multiple(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.matches(kind.clone()) {
                return true;
            }
        }
        return false;
    }

    fn check(&self, kind: TokenKind) -> bool {
        !self.is_at_end() && self.peek().kind == kind
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::EOF
    }

    // ----- Expression Parsing -----
    fn expression(&mut self) -> Result<Expr, ()> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ()> {
        self.binary_op(&[TokenKind::Assign], Parser::or)
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
            Parser::arithmetic,
        )
    }

    fn arithmetic(&mut self) -> Result<Expr, ()> {
        self.binary_op(
            &[TokenKind::Plus, TokenKind::Minus],
            Parser::term,
        )
    }

    fn term(&mut self) -> Result<Expr, ()> {
        self.binary_op(
            &[TokenKind::Star, TokenKind::Slash, TokenKind::Modulus],
            Parser::factor,
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

        self.atom()
    }

    fn atom(&mut self) -> Result<Expr, ()> {
        let tok = self.advance().clone();
        let span = tok.span;

        let kind = match tok.kind {
            TokenKind::NumericLiteral(val) => ExprKind::Literal(LiteralValue::Number(val)),
            TokenKind::StringLiteral(val) => ExprKind::Literal(LiteralValue::String(val)),
            TokenKind::True => ExprKind::Literal(LiteralValue::Boolean(true)),
            TokenKind::False => ExprKind::Literal(LiteralValue::Boolean(false)),
            TokenKind::Identifier(val) => ExprKind::Identifier(val),
            TokenKind::LeftParen => {
                let expr = self.expression()?;

                // Check for closing parentheses
                if !self.matches(TokenKind::RightParen) {
                    todo!("Expected ')' to close set of parentheses");

                    // let peeked = self.peek();

                    // self.error(
                    //     peeked.span,
                    //     ParserErrorKind::UnexpectedToken{
                    //         expected: TokenKind::RightParen,
                    //         found: self.peek().kind.clone(),
                    // });

                }

                // For grouped expressions, we return the inner expression directly.
                // Note: Some people prefer to wrap this in a new Span that 
                // covers from '(' to ')', but returning the inner expr is fine.
                return Ok(expr);
            },
            _ => todo!(),
        };

        return Ok(Expr {
            node: kind,
            span,
        });
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
        let name_token = self.advance().clone();
        let mut span = name_token.span;

        if let TokenKind::Identifier(_) = name_token.kind {
            // Check if we have an optional type annotation
            let var_type = if self.matches(TokenKind::Colon) {
                let type_token = self.advance().clone();
                match type_token.kind {
                    TokenKind::Integer | TokenKind::Float | TokenKind::Boolean | TokenKind::String => {
                        span = span.merge(type_token.span);
                        Some(type_token.kind.clone())
                    },

                    // Invalid type annotation, expected a type keyword
                    _ => todo!("Parser Error: Expected a type after ':'"),
                }
            } else {
                None
            };

            let initializer = if self.matches(TokenKind::Assign) {
                let expr = self.expression()?;
                span = span.merge(expr.span);
                Some(expr)
            } else {
                None
            };


            Ok(Stmt {
                node: StmtKind::VarDeclaration {
                    name: name_token.clone(),
                    var_type,
                    initializer: if initializer.is_some() { initializer.map(Box::new) } else { None },
                },
                span,
            })
        } else {
            todo!("Parser Error: Expected identifier after 'let'");
        }

        
    }
}
