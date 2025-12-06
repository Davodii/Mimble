// Define all moduless
pub mod lexer;
pub mod parser;

pub fn run(code: &str) -> Result<String, String> {
    let src = "something";

    let mut lexer: lexer::Lexer = lexer::Lexer::new(src);
    let tokens = lexer.lex();

    // Parse AST
    let mut parser = parser::Parser::new(tokens);
    if let Ok(stmts) = parser.parse() {
        // now have an AST
        for stmt in stmts {
            stmt.pretty(0);
        }
    }

    // Placeholder implementation
    Ok(format!("Running code:\n{}", code))

}