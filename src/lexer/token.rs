use crate::Location;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub loc: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    NumericLiteral,
    TrueLiteral,
    FalseLiteral,
    StringLiteral,


    Identifier,

    // Keywords
    Let,                        // let
    Integer,                    // int
    Float,                      // float
    Boolean,                    // bool
    String,                     // string

    Do,                         // do
    End,                        // end

    If,                         // if
    Elif,                       // elif
    Else,                       // else
    While,                      // while
    // For,
    // Break,

    Or,                         // or
    And,                        // and
    Not,                        // not
    // In,
    // By, // TODO: Maybe we need this, for range definitons

    // Func,
    // Return,

    // Symbols
    // Comma,                  // ,
    Colon,                      // :
    LeftParen,                  // (
    RightParen,                 // )
    // LeftSquareBracket,      // [
    // RightSquareBracket,     // ]
    // LeftCurlyBracket,           // {
    // RightCurlyBracket,          // }
        
    Assign,                     // =
    Plus,                       // +
    Minus,                      // -
    Star,                       // *
    Slash,                      // /
    Modulus,                    // %
    EQ,                         // ==
    NEQ,                        // !=
    LT,                         // <
    LEQ,                        // <=
    GT,                         // >
    GEQ,                        // >=
    
    EOF,                        // eof
    Error,  
}   