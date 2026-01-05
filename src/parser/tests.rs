

// use core::panic;

// use crate::common::DiagnosticsSink;
// use crate::common::StringPool;

// use super::*;

// fn lex(code: &str) -> Vec<Token> {
//     let mut sink = DiagnosticsSink::new();
//     let mut pool = StringPool::new();
//     let mut lexer = crate::lexer::Lexer::new(code, &mut pool, &mut sink);
//     lexer.lex()
// }

// fn parse(toks: Vec<Token>) -> Vec<Stmt> {
//     let mut sink = DiagnosticsSink::new();
//     let mut pool = StringPool::new();
//     let mut parser = Parser::new(toks, &mut pool, &mut reporter);
//     parser.parse()
// }

// #[test]
// fn test_empty_program() {
//     let toks = lex("");

//     if let Ok(res) = parse(toks) {
//         assert_eq!(res.statements.len(), 0);
//     } else {
//         panic!("Should parse an empty program (only EOF token).");
//     }
// }

// #[test]
// fn test_no_eof() {
//     let toks: Vec<Token> = vec![];

//     if let Ok(_) = parse(toks) {
//         panic!("Parsing should fail if there is no EOF token.");
//     }
// }

// #[test]
// fn test_parse_number_literal() {
//     let toks = lex("123");

//     if let Ok(program) = parse(toks) {
//         assert_eq!(program.statements.len(), 1);
//         if let Spanned{Stmt::ExprStmt(expr), _} = &*program.statements[0] {
//             if let Expr::Literal(LiteralValue::Number(val)) = &**expr {
//                 assert_eq!(*val, 123.0);
//                 return;
//             }
//         }
//     }

//     panic!("Expected to be able to parse.");
// }

// #[test]
// fn test_parse_string_literal() {
//     let toks = lex("\"hello\"");

//     if let Ok(program) = parse(toks) {
//         assert_eq!(program.statements.len(), 1);
//         if let Spanned{StmtKind::ExprStmt(expr), _} = &*program.statements[0] {
//             if let Spanned{ExprKind::Literal(LiteralValue::String(val)), _} = &**expr {
//                 todo!("Check the correct symbol is being used");
                
//                 // assert_eq!(val, "hello");
//                 return;
//             }
//         }
//     }

//     panic!("Expected to be able to parse.");
// }

// #[test]
// fn test_parse_boolean_literal() {
//     let toks = lex("true");

//     if let Ok(program) = parse(toks) {
//         assert_eq!(program.statements.len(), 1);
//         if let Stmt::ExprStmt(expr) = &*program.statements[0] {
//             if let Expr::Literal(LiteralValue::Boolean(val)) = &**expr {
//                 assert_eq!(*val, true);
//                 return;
//             }
//         }
//     }

//     panic!("Expected to be able to parse.");
// }

// #[test]
// fn test_parse_identifier() {
//     let toks = lex("myVar");

//     if let Ok(program) = parse(toks) {
//         assert_eq!(program.statements.len(), 1);
//         if let Stmt::ExprStmt(expr) = &*program.statements[0] {
//             if let Expr::Identifier(token) = &**expr {
//                 assert_eq!(token.lexeme, "myVar");
//                 return;
//             }
//         }
//     }

//     panic!("Expected to be able to parse.");
// }

// #[test]
// fn test_parse_assignment() {
//     let toks = lex("x = 42");

//     if let Ok(program) = parse(toks) {
//         assert_eq!(program.statements.len(), 1);
//         if let Stmt::ExprStmt(expr) = &*program.statements[0] {
//             if let Expr::Assign { name, value } = &**expr {
//                 if let Expr::Identifier(var_token) = &**name {
//                     assert_eq!(var_token.lexeme, "x");
//                 } else {
//                     panic!("Expected variable on left side of assignment.");
//                 }

//                 if let Expr::Literal(LiteralValue::Number(num)) = &**value {
//                     assert_eq!(*num, 42.0);
//                     return;
//                 } else {
//                     panic!("Expected number literal on right side of assignment.");
//                 }
//             }
//         }
//     }

//     panic!("Expected to be able to parse assignment.");
// }


// #[test]
// fn test_parse_declaration_without_type() {
//     let toks = lex("let x = 10");

//     if let Ok(program) = parse(toks) {
//         assert_eq!(program.statements.len(), 1);
//         if let Stmt::VarDeclaration { name, var_type, initializer } = &*program.statements[0] {
//             assert_eq!(name.lexeme, "x");
//             assert!(var_type.is_none());
//             if let Some(expr) = initializer {
//                 if let Expr::Literal(LiteralValue::Number(num)) = &**expr {
//                     assert_eq!(*num, 10.0);
//                     return;
//                 } else {
//                     panic!("Expected number literal as initializer.");
//                 }
//             } else {
//                 panic!("Expected initializer for variable declaration.");
//             }
//         }
//     }

//     panic!("Expected to be able to parse variable declaration without type.");
// }

// #[test]
// fn test_parse_declaration_with_type() {
//     let toks = lex("let y: int = 20");

//     if let Ok(program) = parse(toks) {
//         assert_eq!(program.statements.len(), 1);
//         if let Stmt::VarDeclaration { name, var_type, initializer } = &*program.statements[0] {
//             assert_eq!(name.lexeme, "y");
//             assert!(var_type.is_some());
//             if let Some(TokenKind::Integer) = var_type {
//                 // Correct type
//             } else {
//                 panic!("Expected type to be 'int'.");
//             }
//             if let Some(expr) = initializer {
//                 if let Expr::Literal(LiteralValue::Number(num)) = &**expr {
//                     assert_eq!(*num, 20.0);
//                     return;
//                 } else {
//                     panic!("Expected number literal as initializer.");
//                 }
//             } else {
//                 panic!("Expected initializer for variable declaration.");
//             }
//         }
//     }

//     panic!("Expected to be able to parse variable declaration with type.");
// }

// // TODO: test parsing different types of expressions
// // test operator precedence, parentheses, etc.

// #[test]
// fn test_parse_arithmetic_expression() {
//     let toks = lex("3 + 4 * 2");

//     if let Ok(program) = parse(toks) {
//         assert_eq!(program.statements.len(), 1);
//         if let Stmt::ExprStmt(expr) = &*program.statements[0] {
//             // Expected structure: 3 + (4 * 2)
//             if let Expr::Binary { left, op, right } = &**expr {
//                 assert_eq!(*op, TokenKind::Plus);
//                 if let Expr::Literal(LiteralValue::Number(num)) = &**left {
//                     assert_eq!(*num, 3.0);
//                 } else {
//                     panic!("Expected left operand to be number literal 3.");
//                 }
//                 if let Expr::Binary { left: right_left, op: right_op, right: right_right } = &**right {
//                     assert_eq!(*right_op, TokenKind::Star);
//                     if let Expr::Literal(LiteralValue::Number(num)) = &**right_left {
//                         assert_eq!(*num, 4.0);
//                     } else {
//                         panic!("Expected left operand of multiplication to be number literal 4.");
//                     }
//                     if let Expr::Literal(LiteralValue::Number(num)) = &**right_right {
//                         assert_eq!(*num, 2.0);
//                         return;
//                     } else {
//                         panic!("Expected right operand of multiplication to be number literal 2.");
//                     }
//                 } else {
//                     panic!("Expected right operand to be a multiplication expression.");
//                 }
//             }
//         }
//     }

//     panic!("Expected to be able to parse arithmetic expression.");
// }

// #[test]
// fn test_parse_logical_expression() {
//     let toks = lex("true and false or not false");

//     if let Ok(program) = parse(toks) {
//         assert_eq!(program.statements.len(), 1);
//         if let Stmt::ExprStmt(expr) = &*program.statements[0] {
//             // Expected structure: (true and false) or (not false)
//             if let Expr::Binary { left, op, right } = &**expr {
//                 assert_eq!(*op, TokenKind::Or);
//                 // Check left side: true and false
//                 if let Expr::Binary { left: left_left, op: left_op, right: left_right } = &**left {
//                     assert_eq!(*left_op, TokenKind::And);
//                     if let Expr::Literal(LiteralValue::Boolean(val)) = &**left_left {
//                         assert_eq!(*val, true);
//                     } else {
//                         panic!("Expected left operand of 'and' to be true.");
//                     }
//                     if let Expr::Literal(LiteralValue::Boolean(val)) = &**left_right {
//                         assert_eq!(*val, false);
//                     } else {
//                         panic!("Expected right operand of 'and' to be false.");
//                     }
//                 } else {
//                     panic!("Expected left operand to be an 'and' expression.");
//                 }
//                 // Check right side: not false
//                 if let Expr::Unary { op: right_op, expr: right_expr } = &**right {
//                     assert_eq!(*right_op, TokenKind::Not);
//                     if let Expr::Literal(LiteralValue::Boolean(val)) = &**right_expr {
//                         assert_eq!(*val, false);
//                         return;
//                     } else {
//                         panic!("Expected operand of 'not' to be false.");
//                     }
//                 } else {
//                     panic!("Expected right operand to be a 'not' expression.");
//                 }
//             }
//         }
//     }

//     panic!("Expected to be able to parse logical expression.");
// }

// #[test]
// fn test_parse_grouped_expression() {
//     let toks = lex("(1 + 2) * 3");

//     if let Ok(program) = parse(toks) {
//         assert_eq!(program.statements.len(), 1);
//         if let Stmt::ExprStmt(expr) = &*program.statements[0] {
//             // Expected structure: (1 + 2) * 3
//             if let Expr::Binary { left, op, right } = &**expr {
//                 assert_eq!(*op, TokenKind::Star);
//                 // Check left side: (1 + 2)
//                 if let Expr::Binary { left: left_left, op: left_op, right: left_right } = &**left {
//                     assert_eq!(*left_op, TokenKind::Plus);
//                     if let Expr::Literal(LiteralValue::Number(num)) = &**left_left {
//                         assert_eq!(*num, 1.0);
//                     } else {
//                         panic!("Expected left operand of '+' to be number literal 1.");
//                     }
//                     if let Expr::Literal(LiteralValue::Number(num)) = &**left_right {
//                         assert_eq!(*num, 2.0);
//                     } else {
//                         panic!("Expected right operand of '+' to be number literal 2.");
//                     }
//                 } else {
//                     panic!("Expected left operand to be an addition expression.");
//                 }
//                 // Check right side: 3
//                 if let Expr::Literal(LiteralValue::Number(num)) = &**right {
//                     assert_eq!(*num, 3.0);
//                     return;
//                 } else {
//                     panic!("Expected right operand to be number literal 3.");
//                 }
//             }
//         }
//     }

//     panic!("Expected to be able to parse grouped expression.");
// }

// // #[test]
// // fn test_parse_declaration_invalid() {
// //     let toks = lex("let 123 = 10");

// //     if let Err(err) = parse(toks) {
// //         match err.kind {
// //             ()Kind::UnexpectedToken { token, expected } => {
// //                 assert_eq!(token, "123");
// //                 assert_eq!(expected, "identifier after 'let'");
// //                 return;
// //             },
// //             _ => panic!("Expected UnexpectedToken error."),
// //         }
// //     }

// //     panic!("Expected parsing to fail due to invalid declaration.");
// // }

// // #[test]
// // fn test_parse_assignment_errors() {
// //     let toks = lex("= 10");

// //     if let Err(err) = parse(toks) {
// //         match err.kind {
// //             ()Kind::UnexpectedToken { token, expected } => {
// //                 assert_eq!(token, "Token { kind: Assign, lexeme: \"=\", loc: Location { line: 1, column: 1 } }");
// //                 assert_eq!(expected, "valid expression");
// //                 return;
// //             },
// //             _ => panic!("Expected UnexpectedToken error."),
// //         }
// //     }

// //     panic!("Expected parsing to fail due to invalid assignment.");
// // }

// // #[test]
// // fn test_parse_expression_errors() {
// //     let toks = lex("3 + * 4");

// //     if let Err(err) = parse(toks) {
// //         match err.kind {
// //             ()Kind::UnexpectedToken { token, expected } => {
// //                 assert_eq!(token, "Token { kind: Star, lexeme: \"*\", loc: Location { line: 1, column: 5 } }");
// //                 assert_eq!(expected, "valid expression");
// //                 return;
// //             },
// //             _ => panic!("Expected UnexpectedToken error."),
// //         }
// //     }

// //     panic!("Expected parsing to fail due to invalid expression.");
// // }

// // #[test]
// // fn test_parse_grouping_errors() {
// //     let toks = lex("(1 + 2 * 3");

// //     if let Err(err) = parse(toks) {
// //         match err.kind {
// //             ()Kind::UnexpectedToken { token, expected } => {
// //                 assert_eq!(token, "Token { kind: EOF, lexeme: \"\", loc: Location { line: 1, column: 11 } }");
// //                 assert_eq!(expected, "')' to close set of parentheses");
// //                 return;
// //             },
// //             _ => panic!("Expected UnexpectedToken error."),
// //         }
// //     }

// //     panic!("Expected parsing to fail due to missing closing parenthesis.");
// // }

// // #[test]
// // fn test_parse_declaration_with_type_errors() {
// //     let toks = lex("let x: unknown = 10");

// //     if let Err(err) = parse(toks) {
// //         match err.kind {
// //             ()Kind::UnexpectedToken { token, expected } => {
// //                 assert_eq!(token, "Token { kind: Identifier, lexeme: \"unknown\", loc: Location { line: 1, column: 8 } }");
// //                 assert_eq!(expected, "a type after ':'");
// //                 return;
// //             },
// //             _ => panic!("Expected UnexpectedToken error."),
// //         }
// //     }

// //     panic!("Expected parsing to fail due to invalid type annotation.");
// // }

// #[test]
// fn test_parse_declaration_missing_initializer() {
//     let toks = lex("let x: int");

//     if let Ok(program) = parse(toks) {
//         assert_eq!(program.statements.len(), 1);
//         if let Stmt::VarDeclaration { name, var_type, initializer } = &*program.statements[0] {
//             assert_eq!(name.lexeme, "x");
//             assert!(var_type.is_some());
//             if let Some(TokenKind::Integer) = var_type {
//                 // Correct type
//             } else {
//                 panic!("Expected type to be 'int'.");
//             }
//             assert!(initializer.is_none());
//             return;
//         }
//     }

//     panic!("Expected to be able to parse variable declaration without initializer.");
// }

// #[test]
// fn test_parse_declaration_missing_type_and_initializer() {
//     let toks = lex("let y");

//     if let Ok(program) = parse(toks) {
//         assert_eq!(program.statements.len(), 1);
//         if let Stmt::VarDeclaration { name, var_type, initializer } = &*program.statements[0] {
//             assert_eq!(name.lexeme, "y");
//             assert!(var_type.is_none());
//             assert!(initializer.is_none());
//             return;
//         }
//     }

//     panic!("Expected to be able to parse variable declaration without type and initializer.");
// }

// #[test]
// fn test_parse_complex_expression() {
//     let toks = lex("a + b * (c - d) / e and not f or g");
//     if let Ok(program) = parse(toks) {
//         assert_eq!(program.statements.len(), 1);

//         // Get the debug printout
//         let stmt = &program.statements[0];
//         let str = stmt.to_test_string();

//         // TODO: check this is correct
//         assert_eq!(str, "(ExprStmt (Binary (Binary (Binary (Variable a) Plus (Binary (Binary (Variable b) Star (Binary (Variable c) Minus (Variable d))) Slash (Variable e))) And (Unary Not (Variable f))) Or (Variable g)))");
//         return;
//     }
//     panic!("Expected to be able to parse complex expression.");
// }

// #[test]
// fn test_parse_nested_groupings() {
//     let toks = lex("((1 + 2) * (3 - 4)) / (5 + (6 * 7))");
//     if let Ok(program) = parse(toks) {
//         assert_eq!(program.statements.len(), 1);
//         let stmt = &program.statements[0];
//         let str = stmt.to_test_string();
//         assert_eq!(str, "(ExprStmt (Binary (Binary (Binary (Number 1) Plus (Number 2)) Star (Binary (Number 3) Minus (Number 4))) Slash (Binary (Number 5) Plus (Binary (Number 6) Star (Number 7)))))");
//         return;
//     }
//     panic!("Expected to be able to parse nested groupings.");
// }

// #[test]
// fn test_parse_logical_precedence() {
//     let toks = lex("true or false and not false");
//     if let Ok(program) = parse(toks) {
//         assert_eq!(program.statements.len(), 1);
//         let stmt = &program.statements[0];
//         let str = stmt.to_test_string();
//         assert_eq!(str, "(ExprStmt (Binary (Boolean true) Or (Binary (Boolean false) And (Unary Not (Boolean false)))))");
//         return;
//     }
//     panic!("Expected to be able to parse logical expression with correct precedence.");
// }

// #[test]
// fn test_parse_assignment_chain() {
//     let toks = lex("a = b = c = 10");
//     if let Ok(program) = parse(toks) {
//         assert_eq!(program.statements.len(), 1);
//         let stmt = &program.statements[0];
//         let str = stmt.to_test_string();
//         println!("{}", str);
//         assert_eq!(str, "(ExprStmt (Assign (Variable a) (Assign (Variable b) (Assign (Variable c) (Number 10)))))");
//         return;
//     }
//     panic!("Expected to be able to parse chained assignments.");
// }

// #[test]
// fn test_parse_unary_operations() {
//     let toks = lex("-a + +b - -c");
//     if let Ok(program) = parse(toks) {
//         assert_eq!(program.statements.len(), 1);
//         let stmt = &program.statements[0];
//         let str = stmt.to_test_string();
//         assert_eq!(str, "(ExprStmt (Binary (Binary (Unary Minus (Variable a)) Plus (Unary Plus (Variable b))) Minus (Unary Minus (Variable c))))");
//         return;
//     }
//     panic!("Expected to be able to parse unary operations.");
// }

// #[test]
// fn test_parse_precedence_with_unary() {
//     let toks = lex("not a and b or -c + d * e");
//     if let Ok(program) = parse(toks) {
//         assert_eq!(program.statements.len(), 1);
//         let stmt = &program.statements[0];
//         let str = stmt.to_test_string();

//         assert_eq!(str, "(ExprStmt (Binary (Binary (Unary Not (Variable a)) And (Variable b)) Or (Binary (Unary Minus (Variable c)) Plus (Binary (Variable d) Star (Variable e)))))");
//         return;
//     }
//     panic!("Expected to be able to parse expression with unary and precedence.");
// }

// #[test]
// fn test_parse_if_stmt() {
//     todo!()
// }

// #[test]
// fn test_parse_while_stmt() {
//     todo!()
// }

// #[test]
// fn test_parse_block() {
//     todo!()
// }
