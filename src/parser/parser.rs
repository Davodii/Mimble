use super::ast::{Expr, Stmt, LiteralValue, Program};
use crate::{Location, lexer::{Token, TokenKind}};
use super::error::{ParserError, ParserErrorKind};

pub struct Parser{
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, ParserError> {
        if self.tokens.is_empty() || self.tokens.last().unwrap().kind != TokenKind::EOF {
            return Err(self.error(ParserErrorKind::UnexpectedEOF));
        }

        let mut statements = Vec::new();
        while !self.is_at_end(){
            statements.push(self.statement()?);
        }

        Ok(Program { statements })
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

    fn error(&self, kind: ParserErrorKind) -> ParserError {
        let location = if self.tokens.is_empty() {
            Location { line: 0, column: 0 }
        } else if self.current >= self.tokens.len() {
            self.tokens.last().unwrap().loc.clone()
        } else {
            self.peek().loc.clone()
        };

        ParserError {
            kind,
            location,
        }
    }

    // ----- Expression Parsing -----
    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParserError> {
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

    fn or(&mut self) -> Result<Expr, ParserError> {
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

    fn and(&mut self) -> Result<Expr, ParserError> {
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

    fn not(&mut self) -> Result<Expr, ParserError> {
        if self.matches(TokenKind::Not) {
            let expr = self.not()?;
            return Ok(Expr::Unary { 
                op: TokenKind::Not, 
                expr: Box::new(expr) 
            });
        }

        self.comparison()
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
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

    fn arithmetic(&mut self) -> Result<Expr, ParserError> {
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

    fn term(&mut self) -> Result<Expr, ParserError> {
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

    fn factor(&mut self) -> Result<Expr, ParserError> {
        if self.matches_multiple(&[TokenKind::Plus, TokenKind::Minus]) {
            let op = self.previous().kind.clone();
            let right = self.factor()?;
            return Ok(Expr::Unary { op, expr: Box::new(right) });
        }

        self.atom()
    }

    fn atom(&mut self) -> Result<Expr, ParserError> {
        let tok = self.advance().clone();

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
                    Err(self.error(ParserErrorKind::UnexpectedToken{
                        token: format!("{:?}", self.peek()),
                        expected: "')' to close set of parentheses".to_string(),
                    }))
                } else {
                    Ok(expr)
                } 
            },
            _ => Err(self.error(ParserErrorKind::UnexpectedToken{
                token: format!("{:?}", tok),
                expected: "valid expression".to_string(),
            })),
        }
    }

    // ----- Statement Parsing -----
    fn statement(&mut self) -> Result<Stmt, ParserError> {
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
            Ok(Stmt::ExprStmt(expr))
        }
    }

    fn if_stmt(&mut self) -> Result<Stmt, ParserError> {
        todo!()
    }

    fn while_stmt(&mut self) -> Result<Stmt, ParserError> {
        todo!()
    }

    fn declaration(&mut self) -> Result<Stmt, ParserError> {
        let name_token = self.advance().clone();
        if name_token.kind != TokenKind::Identifier {
            return Err(self.error(ParserErrorKind::UnexpectedToken{
                token: name_token.lexeme.clone(),
                expected: "identifier after 'let'".to_string(),
            }));
        }

        // Check if we have an optional type annotation
        let var_type = if self.matches(TokenKind::Colon) {
            let type_token = self.advance().clone();
            match type_token.kind {
                TokenKind::Integer | TokenKind::Float | TokenKind::Boolean | TokenKind::String => {
                    Some(type_token.kind.clone())
                },
                _ => {
                    return Err(self.error(ParserErrorKind::UnexpectedToken{
                        token: format!("{:?}", type_token),
                        expected: "a type after ':'".to_string(),
                    }));
                }
            }
        } else {
            None
        };

        let initializer = if self.matches(TokenKind::Assign) {
            Some(self.expression()?)
        } else {
            None
        };

        Ok(Stmt::VarDeclaration {
            name: name_token.clone(),
            var_type,
            initializer,
        })
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    fn lex(code: &str) -> Vec<Token> {
        let mut lexer = crate::lexer::Lexer::new(code);
        lexer.lex().unwrap()
    }

    fn parse(toks: Vec<Token>) -> Result<Program, ParserError> {
        let mut parser = Parser::new(toks);
        parser.parse()
    }

    #[test]
    fn test_empty_program() {
        let toks = lex("");

        if let Ok(res) = parse(toks) {
            assert_eq!(res.statements.len(), 0);
        } else {
            panic!("Should parse an empty program (only EOF token).");
        }
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
        let toks = lex("123");

        if let Ok(program) = parse(toks) {
            assert_eq!(program.statements.len(), 1);
            if let Stmt::ExprStmt(expr) = &program.statements[0] {
                if let Expr::Literal(LiteralValue::Number(val)) = expr {
                    assert_eq!(*val, 123.0);
                    return;
                }
            }
        }

        panic!("Expected to be able to parse.");
    }

    #[test]
    fn test_parse_string_literal() {
        let toks = lex("\"hello\"");

        if let Ok(program) = parse(toks) {
            assert_eq!(program.statements.len(), 1);
            if let Stmt::ExprStmt(expr) = &program.statements[0] {
                if let Expr::Literal(LiteralValue::String(val)) = expr {
                    assert_eq!(val, "hello");
                    return;
                }
            }
        }

        panic!("Expected to be able to parse.");
    }

    #[test]
    fn test_parse_boolean_literal() {
        let toks = lex("true");

        if let Ok(program) = parse(toks) {
            assert_eq!(program.statements.len(), 1);
            if let Stmt::ExprStmt(expr) = &program.statements[0] {
                if let Expr::Literal(LiteralValue::Boolean(val)) = expr {
                    assert_eq!(*val, true);
                    return;
                }
            }
        }

        panic!("Expected to be able to parse.");
    }

    #[test]
    fn test_parse_identifier() {
        let toks = lex("myVar");

        if let Ok(program) = parse(toks) {
            assert_eq!(program.statements.len(), 1);
            if let Stmt::ExprStmt(expr) = &program.statements[0] {
                if let Expr::Variable(token) = expr {
                    assert_eq!(token.lexeme, "myVar");
                    return;
                }
            }
        }

        panic!("Expected to be able to parse.");
    }

    #[test]
    fn test_parse_assignment() {
        let toks = lex("x = 42");

        if let Ok(program) = parse(toks) {
            assert_eq!(program.statements.len(), 1);
            if let Stmt::ExprStmt(expr) = &program.statements[0] {
                if let Expr::Assign { name, value } = expr {
                    if let Expr::Variable(var_token) = &**name {
                        assert_eq!(var_token.lexeme, "x");
                    } else {
                        panic!("Expected variable on left side of assignment.");
                    }

                    if let Expr::Literal(LiteralValue::Number(num)) = &**value {
                        assert_eq!(*num, 42.0);
                        return;
                    } else {
                        panic!("Expected number literal on right side of assignment.");
                    }
                }
            }
        }

        panic!("Expected to be able to parse assignment.");
    }


    #[test]
    fn test_parse_declaration_without_type() {
        let toks = lex("let x = 10");

        if let Ok(program) = parse(toks) {
            assert_eq!(program.statements.len(), 1);
            if let Stmt::VarDeclaration { name, var_type, initializer } = &program.statements[0] {
                assert_eq!(name.lexeme, "x");
                assert!(var_type.is_none());
                if let Some(expr) = initializer {
                    if let Expr::Literal(LiteralValue::Number(num)) = expr {
                        assert_eq!(*num, 10.0);
                        return;
                    } else {
                        panic!("Expected number literal as initializer.");
                    }
                } else {
                    panic!("Expected initializer for variable declaration.");
                }
            }
        }

        panic!("Expected to be able to parse variable declaration without type.");
    }

    #[test]
    fn test_parse_declaration_with_type() {
        let toks = lex("let y: int = 20");

        if let Ok(program) = parse(toks) {
            assert_eq!(program.statements.len(), 1);
            if let Stmt::VarDeclaration { name, var_type, initializer } = &program.statements[0] {
                assert_eq!(name.lexeme, "y");
                assert!(var_type.is_some());
                if let Some(TokenKind::Integer) = var_type {
                    // Correct type
                } else {
                    panic!("Expected type to be 'int'.");
                }
                if let Some(expr) = initializer {
                    if let Expr::Literal(LiteralValue::Number(num)) = expr {
                        assert_eq!(*num, 20.0);
                        return;
                    } else {
                        panic!("Expected number literal as initializer.");
                    }
                } else {
                    panic!("Expected initializer for variable declaration.");
                }
            }
        }

        panic!("Expected to be able to parse variable declaration with type.");
    }

    // TODO: test parsing different types of expressions
    // test operator precedence, parentheses, etc.

    #[test]
    fn test_parse_arithmetic_expression() {
        let toks = lex("3 + 4 * 2");

        if let Ok(program) = parse(toks) {
            assert_eq!(program.statements.len(), 1);
            if let Stmt::ExprStmt(expr) = &program.statements[0] {
                // Expected structure: 3 + (4 * 2)
                if let Expr::Binary { left, op, right } = expr {
                    assert_eq!(*op, TokenKind::Plus);
                    if let Expr::Literal(LiteralValue::Number(num)) = &**left {
                        assert_eq!(*num, 3.0);
                    } else {
                        panic!("Expected left operand to be number literal 3.");
                    }
                    if let Expr::Binary { left: right_left, op: right_op, right: right_right } = &**right {
                        assert_eq!(*right_op, TokenKind::Star);
                        if let Expr::Literal(LiteralValue::Number(num)) = &**right_left {
                            assert_eq!(*num, 4.0);
                        } else {
                            panic!("Expected left operand of multiplication to be number literal 4.");
                        }
                        if let Expr::Literal(LiteralValue::Number(num)) = &**right_right {
                            assert_eq!(*num, 2.0);
                            return;
                        } else {
                            panic!("Expected right operand of multiplication to be number literal 2.");
                        }
                    } else {
                        panic!("Expected right operand to be a multiplication expression.");
                    }
                }
            }
        }

        panic!("Expected to be able to parse arithmetic expression.");
    }

    #[test]
    fn test_parse_logical_expression() {
        let toks = lex("true and false or not false");

        if let Ok(program) = parse(toks) {
            assert_eq!(program.statements.len(), 1);
            if let Stmt::ExprStmt(expr) = &program.statements[0] {
                // Expected structure: (true and false) or (not false)
                if let Expr::Binary { left, op, right } = expr {
                    assert_eq!(*op, TokenKind::Or);
                    // Check left side: true and false
                    if let Expr::Binary { left: left_left, op: left_op, right: left_right } = &**left {
                        assert_eq!(*left_op, TokenKind::And);
                        if let Expr::Literal(LiteralValue::Boolean(val)) = &**left_left {
                            assert_eq!(*val, true);
                        } else {
                            panic!("Expected left operand of 'and' to be true.");
                        }
                        if let Expr::Literal(LiteralValue::Boolean(val)) = &**left_right {
                            assert_eq!(*val, false);
                        } else {
                            panic!("Expected right operand of 'and' to be false.");
                        }
                    } else {
                        panic!("Expected left operand to be an 'and' expression.");
                    }
                    // Check right side: not false
                    if let Expr::Unary { op: right_op, expr: right_expr } = &**right {
                        assert_eq!(*right_op, TokenKind::Not);
                        if let Expr::Literal(LiteralValue::Boolean(val)) = &**right_expr {
                            assert_eq!(*val, false);
                            return;
                        } else {
                            panic!("Expected operand of 'not' to be false.");
                        }
                    } else {
                        panic!("Expected right operand to be a 'not' expression.");
                    }
                }
            }
        }

        panic!("Expected to be able to parse logical expression.");
    }

    #[test]
    fn test_parse_grouped_expression() {
        let toks = lex("(1 + 2) * 3");

        if let Ok(program) = parse(toks) {
            assert_eq!(program.statements.len(), 1);
            if let Stmt::ExprStmt(expr) = &program.statements[0] {
                // Expected structure: (1 + 2) * 3
                if let Expr::Binary { left, op, right } = expr {
                    assert_eq!(*op, TokenKind::Star);
                    // Check left side: (1 + 2)
                    if let Expr::Binary { left: left_left, op: left_op, right: left_right } = &**left {
                        assert_eq!(*left_op, TokenKind::Plus);
                        if let Expr::Literal(LiteralValue::Number(num)) = &**left_left {
                            assert_eq!(*num, 1.0);
                        } else {
                            panic!("Expected left operand of '+' to be number literal 1.");
                        }
                        if let Expr::Literal(LiteralValue::Number(num)) = &**left_right {
                            assert_eq!(*num, 2.0);
                        } else {
                            panic!("Expected right operand of '+' to be number literal 2.");
                        }
                    } else {
                        panic!("Expected left operand to be an addition expression.");
                    }
                    // Check right side: 3
                    if let Expr::Literal(LiteralValue::Number(num)) = &**right {
                        assert_eq!(*num, 3.0);
                        return;
                    } else {
                        panic!("Expected right operand to be number literal 3.");
                    }
                }
            }
        }

        panic!("Expected to be able to parse grouped expression.");
    }

    #[test]
    fn test_parse_declaration_errors() {
        let toks = lex("let 123 = 10");

        if let Err(err) = parse(toks) {
            match err.kind {
                ParserErrorKind::UnexpectedToken { token, expected } => {
                    assert_eq!(token, "Token { kind: NumericLiteral, lexeme: \"123\", loc: Location { line: 1, column: 5 } }");
                    assert_eq!(expected, "identifier after 'let'");
                    return;
                },
                _ => panic!("Expected UnexpectedToken error."),
            }
        }

        panic!("Expected parsing to fail due to invalid declaration.");
    }

    #[test]
    fn test_parse_assignment_errors() {
        let toks = lex("= 10");

        if let Err(err) = parse(toks) {
            match err.kind {
                ParserErrorKind::UnexpectedToken { token, expected } => {
                    assert_eq!(token, "Token { kind: Assign, lexeme: \"=\", loc: Location { line: 1, column: 1 } }");
                    assert_eq!(expected, "valid expression");
                    return;
                },
                _ => panic!("Expected UnexpectedToken error."),
            }
        }

        panic!("Expected parsing to fail due to invalid assignment.");
    }

    #[test]
    fn test_parse_expression_errors() {
        let toks = lex("3 + * 4");

        if let Err(err) = parse(toks) {
            match err.kind {
                ParserErrorKind::UnexpectedToken { token, expected } => {
                    assert_eq!(token, "Token { kind: Star, lexeme: \"*\", loc: Location { line: 1, column: 5 } }");
                    assert_eq!(expected, "valid expression");
                    return;
                },
                _ => panic!("Expected UnexpectedToken error."),
            }
        }

        panic!("Expected parsing to fail due to invalid expression.");
    }

    #[test]
    fn test_parse_grouping_errors() {
        let toks = lex("(1 + 2 * 3");

        if let Err(err) = parse(toks) {
            match err.kind {
                ParserErrorKind::UnexpectedToken { token, expected } => {
                    assert_eq!(token, "Token { kind: EOF, lexeme: \"\", loc: Location { line: 1, column: 11 } }");
                    assert_eq!(expected, "')' to close set of parentheses");
                    return;
                },
                _ => panic!("Expected UnexpectedToken error."),
            }
        }

        panic!("Expected parsing to fail due to missing closing parenthesis.");
    }

    #[test]
    fn test_parse_declaration_with_type_errors() {
        let toks = lex("let x: unknown = 10");

        if let Err(err) = parse(toks) {
            match err.kind {
                ParserErrorKind::UnexpectedToken { token, expected } => {
                    assert_eq!(token, "Token { kind: Identifier, lexeme: \"unknown\", loc: Location { line: 1, column: 8 } }");
                    assert_eq!(expected, "a type after ':'");
                    return;
                },
                _ => panic!("Expected UnexpectedToken error."),
            }
        }

        panic!("Expected parsing to fail due to invalid type annotation.");
    }

    #[test]
    fn test_parse_declaration_missing_initializer() {
        let toks = lex("let x: int");

        if let Ok(program) = parse(toks) {
            assert_eq!(program.statements.len(), 1);
            if let Stmt::VarDeclaration { name, var_type, initializer } = &program.statements[0] {
                assert_eq!(name.lexeme, "x");
                assert!(var_type.is_some());
                if let Some(TokenKind::Integer) = var_type {
                    // Correct type
                } else {
                    panic!("Expected type to be 'int'.");
                }
                assert!(initializer.is_none());
                return;
            }
        }

        panic!("Expected to be able to parse variable declaration without initializer.");
    }

    #[test]
    fn test_parse_declaration_missing_type_and_initializer() {
        let toks = lex("let y");

        if let Ok(program) = parse(toks) {
            assert_eq!(program.statements.len(), 1);
            if let Stmt::VarDeclaration { name, var_type, initializer } = &program.statements[0] {
                assert_eq!(name.lexeme, "y");
                assert!(var_type.is_none());
                assert!(initializer.is_none());
                return;
            }
        }

        panic!("Expected to be able to parse variable declaration without type and initializer.");
    }

    #[test]
    fn test_parse_complex_expression() {
        let toks = lex("a + b * (c - d) / e and not f or g");
        if let Ok(program) = parse(toks) {
            assert_eq!(program.statements.len(), 1);
            // Further detailed checks can be added here to verify the structure
            // of the parsed expression tree.
            todo!()
        }
        panic!("Expected to be able to parse complex expression.");
    }

    #[test]
    fn test_parse_nested_groupings() {
        let toks = lex("((1 + 2) * (3 - 4)) / (5 + (6 * 7))");
        if let Ok(program) = parse(toks) {
            assert_eq!(program.statements.len(), 1);
            // Further detailed checks can be added here to verify the structure
            // of the parsed expression tree.
            todo!()
        }
        panic!("Expected to be able to parse nested groupings.");
    }

    #[test]
    fn test_parse_logical_precedence() {
        let toks = lex("true or false and not false");
        if let Ok(program) = parse(toks) {
            assert_eq!(program.statements.len(), 1);
            // Further detailed checks can be added here to verify the structure
            // of the parsed expression tree.
            todo!()
        }
        panic!("Expected to be able to parse logical expression with correct precedence.");
    }

    #[test]
    fn test_parse_assignment_chain() {
        let toks = lex("a = b = c = 10");
        if let Ok(program) = parse(toks) {
            assert_eq!(program.statements.len(), 1);
            // Further detailed checks can be added here to verify the structure
            // of the parsed expression tree.
            todo!()
        }
        panic!("Expected to be able to parse chained assignments.");
    }

    #[test]
    fn test_parse_unary_operations() {
        let toks = lex("-a + +b - -c");
        if let Ok(program) = parse(toks) {
            assert_eq!(program.statements.len(), 1);
            // Further detailed checks can be added here to verify the structure
            // of the parsed expression tree.
            todo!()
        }
        panic!("Expected to be able to parse unary operations.");
    }

    #[test]
    fn test_parse_precedence_with_unary() {
        let toks = lex("not a and b or -c + d * e");
        if let Ok(program) = parse(toks) {
            assert_eq!(program.statements.len(), 1);
            // Further detailed checks can be added here to verify the structure
            // of the parsed expression tree.
            todo!()
        }
        panic!("Expected to be able to parse expression with unary and precedence.");
    }

    #[test]
    fn test_parse_if_stmt() {
        todo!()
    }

    #[test]
    fn test_parse_while_stmt() {
        todo!()
    }

    #[test]
    fn test_parse_block() {
        todo!()
    }

}