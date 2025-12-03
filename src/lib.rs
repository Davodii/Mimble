use crate::lexer::Lexer;

pub mod lexer;

pub fn run(code: &str) -> Result<String, String> {
    let src = "something";

    let mut lexer: Lexer = Lexer::new(src);
    lexer.lex();

    // Placeholder implementation
    Ok(format!("Running code:\n{}", code))

}