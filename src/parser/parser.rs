use super::ast::{Expr, Stmt, LiteralValue};
use super::error::ParseError;
use crate::lexer::{Token, TokenKind};

pub struct Parser{
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }

    // ----- Expression Parsing -----
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;

        if self.matches(TokenKind::Assign) {
            let value = self.assignment()?;
            return Ok(Expr::Assign {
                name: Box::new(expr),
                value: Box::new(value),
            });
        } 

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;

        while self.matches(TokenKind::Or) {
            let right = self.and()?;
            expr = Expr::Binary { 
                left: Box::new(expr), 
                op: TokenKind::Or, 
                right: Box::new(right)
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.not()?;

        while self.matches(TokenKind::And) {
            let right = self.not()?;
            expr = Expr::Binary { 
                left: Box::new(expr), 
                op: TokenKind::And, 
                right: Box::new(right)
            };
        }

        Ok(expr)
    }

    fn not(&mut self) -> Result<Expr, ParseError> {
        if self.matches(TokenKind::Not) {
            let expr = self.not()?;
            return Ok(Expr::Unary { 
                op: TokenKind::Not, 
                expr: Box::new(expr) 
            });
        }

        self.comparison()
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.arithmetic()?;

        while self.matches_multiple(&[
            TokenKind::LT, TokenKind::LEQ,
            TokenKind::GT, TokenKind::GEQ,
            TokenKind::EQ, TokenKind::NEQ,
        ]) {
            let op = self.previous().kind.clone();
            let right = self.arithmetic()?;
            expr = Expr::Binary { 
                left: Box::new(expr), 
                op, 
                right: Box::new(right) 
            };
        }

        Ok(expr)
    }

    fn arithmetic(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.matches_multiple(&[
            TokenKind::Plus, TokenKind::Minus
        ]) {
            let op = self.previous().kind.clone();
            let right = self.term()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.matches_multiple(&[
            TokenKind::Star, TokenKind::Slash, TokenKind::Modulus,
        ]) {
            let op = self.previous().kind.clone();
            let right = self.factor()?;
            expr = Expr::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        if self.matches_multiple(&[TokenKind::Plus, TokenKind::Minus]) {
            let op = self.previous().kind.clone();
            let right = self.factor()?;
            return Ok(Expr::Unary { op, expr: Box::new(right) });
        }

        self.atom()
    }

    fn atom(&mut self) -> Result<Expr, ParseError> {
        let tok = self.advance();

        match tok.kind {
            TokenKind::NumericLiteral => {
                let value: f64 = tok.lexeme.parse().unwrap();
                Ok(Expr::Literal(LiteralValue::Number(value)))
            },
            TokenKind::StringLiteral => {
                Ok(Expr::Literal(LiteralValue::String(tok.lexeme.clone())))
            },
            TokenKind::TrueLiteral => Ok(Expr::Literal(LiteralValue::Boolean(true))),
            TokenKind::FalseLiteral => Ok(Expr::Literal(LiteralValue::Boolean(false))),
            TokenKind::Identifier => Ok(Expr::Variable(tok.clone())),
            TokenKind::LeftParen => {
                let expr = self.expression()?;
                if !self.matches(TokenKind::RightParen) {
                    panic!("Expected ')'");
                }
                Ok(expr) 
            },
            _ => Err(ParseError { message: format!("Unexpected token: {:?}", tok), line: tok.line, column: tok.column }),
        }
    }

    // ----- Statement Parsing -----
    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.matches(TokenKind::If) {
            self.if_stmt()
        } else if self.matches(TokenKind::While) {
            self.while_stmt()
        } else {
            // fallback: expression statement
            let expr = self.expression()?;
            Ok(Stmt::ExprStmt(expr))
        }
    }

    fn if_stmt(&mut self) -> Result<Stmt, ParseError> {
        Err(ParseError {
            message: "Expected if statement".to_owned(),
            line: 0,
            column: 0,
        })
    }

    fn while_stmt(&mut self) -> Result<Stmt, ParseError> {
        Err(ParseError { 
            message: "Expected while statement".to_owned(), 
            line: 0, 
            column: 0 })
    }

    // ----- Utilities -----
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
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    fn eof() -> Token {
        Token {
            kind: TokenKind::EOF,
            lexeme: "".to_owned(),
            line: 0,
            column: 0,
        }
    }

    fn parse(toks: Vec<Token>) -> Result<Vec<Stmt>, ParseError> {
        let mut parser = Parser::new(toks);
        parser.parse()
    }

    #[test]
    fn test_empty_program() {
        let toks = vec![eof()];

        if let Ok(res) = parse(toks) {
            assert_eq!(res.len(), 0);
        } 

        panic!("Should parse an empty program (only EOF token).");
    }

    #[test]
    fn test_no_eof() {
        let toks: Vec<Token> = vec![];

        if let Ok(_) = parse(toks) {
            panic!("Parsing should fail if there is no EOF token.");
        }
    }

    #[test]
    fn test_parse_number_literal() {
        let toks = vec![
            Token {
                kind: TokenKind::NumericLiteral,
                lexeme: "123".to_string(),
                line: 0, column: 0,
            },
            eof(),
        ];

        todo!()
    }

    #[test]
    fn test_parse_string_literal() {
        todo!()
    }

    #[test]
    fn test_parse_boolean_literal() {
        todo!()
    }

    #[test]
    fn test_parse_identifier() {
        todo!()
    }

    #[test]
    fn test_parse_assignment() {
        todo!()
    }

    // TODO: test parsing different types of expressions

    #[test]
    fn test_parse_if_stmt() {
        todo!()
    }

    #[test]
    fn test_parse_while_stmt() {
        todo!()
    }

    fn test_parse_block() {
        todo!()
    }

}