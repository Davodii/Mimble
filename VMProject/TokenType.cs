namespace VMProject;

public enum TokenType
{
    Eol,                    // similar to semicolon in C#
    Eof,
    Error,
    
    // SINGLE CHAR TOKENS
    LeftParen,              // (
    RightParen,             // )
    SqLeftBrace,            // [
    SqRightBrace,           // ]
    Comma,                  // ,
    Minus,                  // -
    Plus,                   // +
    Slash,                  // /
    Star,                   // *
    
    // ONE OR MORE CHAR TOKENS
    Equal,                  // =
    EqualEqual,             // ==
    Greater,                // >
    GreaterEqual,           // >=
    Less,                   // <
    LessEqual,              // <=
    Colon,                  // :
    DoubleDot,              // ..
    
    // LITERALS
    Identifier,
    String,
    Number,
    Range,
    List,
    Null,
    
    // KEYWORDS
    And, Or, Not, True, False,
    If, Elif, Else, While, For, 
    In, Do, Does, End, Break,
    Continue, Function, Return
}