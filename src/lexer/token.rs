
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: crate::common::Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Let, Integer, Float, Boolean, String, Do, End, 
    If, Elif, Else, While, 
    // For, Break, Continue,
    // In, By, Func, Return,
    Or,                         // or
    And,                        // and
    Not,                        // not

    // Symbols
    Comma, Colon, LeftParen, RightParen,
    LeftSquareBracket,      // [
    RightSquareBracket,     // ]
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
    IntegerLiteral(i64),
    FloatLiteral(f64),
    True,
    False,
    StringLiteral(String),
    Identifier(crate::common::Symbol),
    
    Error,  
}   

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            TokenKind::IntegerLiteral(n) => write!(f, "IntegerLiteral({})", n),
            TokenKind::FloatLiteral(n) => write!(f, "FloatLiteral({})", n),
            TokenKind::StringLiteral(_) => write!(f, "StringLiteral"),
            TokenKind::Identifier(_) => write!(f, "Identifier"),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::Assign => write!(f, "="),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Modulus => write!(f, "%"),
            TokenKind::EQ => write!(f, "=="),
            TokenKind::NEQ => write!(f, "!="),
            TokenKind::LT => write!(f, "<"),
            TokenKind::LEQ => write!(f, "<="),
            TokenKind::GT => write!(f, ">"),
            TokenKind::GEQ => write!(f, ">="),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftSquareBracket => write!(f, "["),
            TokenKind::RightSquareBracket => write!(f, "]"),
            TokenKind::Let => write!(f, "let"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Elif => write!(f, "elif"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::While => write!(f, "while"),
            // TokenKind::For => write!(f, "for"),
            // TokenKind::In => write!(f, "in"),
            // TokenKind::By => write!(f, "by"),
            // TokenKind::Func => write!(f, "func"),
            // TokenKind::Return => write!(f, "return"),
            TokenKind::Do => write!(f, "do"),
            TokenKind::End => write!(f, "end"),
            TokenKind::Or => write!(f, "or"),
            TokenKind::And => write!(f, "and"),
            TokenKind::Not => write!(f, "not"),
            // TokenKind::Break => write!(f, "break"),
            // TokenKind::Continue => write!(f, "continue"),
            _ => write!(f, "{:?}", self.kind),
        }
    }
}