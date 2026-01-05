
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: crate::common::Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Let, Integer, Float, Boolean, String, Do, End, 
    If, Elif, Else, While, For, Break, Continue,
    In, By, Func, Return,
    Or,                         // or
    And,                        // and
    Not,                        // not

    // Symbols
    Comma, Colon, LeftParen, RightParen,
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

    // Literals
    NumericLiteral(f64),
    True,
    False,
    StringLiteral(crate::common::Symbol),
    Identifier(crate::common::Symbol),
    
    Error,  
}   