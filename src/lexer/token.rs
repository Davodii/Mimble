
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,   // index of start byte
    pub end: usize,     // index of end byte
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literal types
    NumericLiteral,
    TrueLiteral,
    FalseLiteral,
    StringLiteral,

    Identifier,

    // Keywords
    Do,
    End,

    If,
    Elif,
    Else,
    While,
    // For,
    // Break,

    Or,
    And,
    Not,
    // In,
    // By, // TODO: Maybe we need this, for range definitons

    // Func,
    // Return,

    // Symbols
    // Comma,                  // ,
    // Colon,                  // :
    LeftParen,              // (
    RightParen,             // )
    // LeftSquareBracket,      // [
    // RightSqquareBracket,    // ]
    // LeftCurlyBracket,       // {
    // RightCurlyBracket,      // }
    
    Assign,                 // =
    Plus,                   // +
    Minus,                  // -
    Star,                   // *
    Slash,                  // /
    Modulus,                // %
    EQ,                     // ==
    NEQ,                    // !=
    LT,                     // <
    LEQ,                    // <=
    GT,                     // >
    GEQ,                    // >=

    EOF,                    // eof
    Error,
}