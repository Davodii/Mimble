// Lexer for the language
mod token;
mod error;

pub use token::{Token, TokenKind};
use crate::common::{DiagnosticsSink, StringPool};

// TODO: Implement proper error reporting using DiagnosticsSink

pub struct Lexer<'a> {
    src: &'a str,
    pool: &'a mut StringPool,
    sink: &'a mut DiagnosticsSink,
    current: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new( 
        src: &'a str,
        pool: &'a mut crate::common::StringPool,
        sink: &'a mut crate::common::DiagnosticsSink
    ) -> Self {
        Self {
            src,
            pool,
            sink,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    // fn error(&mut self, kind: LexerErrorKind) -> LexerError {
    //     let err = LexerError {
    //         kind,
    //         location: crate::common::Span {
    //             start: self.current,
    //             end: self.current,
    //             line: self.line,
    //             column: self.column,
    //         },
    //     };

    //     let diag = err.to_diagnostic();
    //     self.sink.report(diag);

    //     err
    // }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            if let Some(token) = self.scan_token() {
                tokens.push(token);
            }
        }

        tokens.push(self.make_at(TokenKind::EOF, self.current, self.line, self.column));
        
        tokens
    }

    fn scan_token(&mut self) -> Option<Token> {
        // Skip whitespace
        self.skip_whitespace();

        // Capture start
        let line = self.line;
        let column = self.column;
        let start = self.current;

        let c = self.advance()?;

        let kind = match c {
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            ',' => TokenKind::Comma,
            '[' => TokenKind::LeftSquareBracket,
            ']' => TokenKind::RightSquareBracket,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Modulus,
            ':' => TokenKind::Colon,
            '!' => {
                if self.matches('=') { TokenKind::NEQ }
                else { TokenKind::Error }
            },

            '=' => {
                if self.matches('=') { TokenKind::EQ }
                else { TokenKind::Assign }
            },

            '<' => {
                if self.matches('=') { TokenKind::LEQ }
                else { TokenKind::LT }
            },

            '>' => {
                if self.matches('=') { TokenKind::GEQ }
                else {TokenKind::GT }
            },

            '"' => return Some(self.string(start, line, column)),

            c if c.is_ascii_digit() => {
                return Some(self.number(start, line, column));
            },

            c if is_ident_start(c) => {
                return Some(self.identifier(start, line, column));
            },

            _ => {
                // Unknown character
                TokenKind::Error
            },
        };

        Some(self.make_at(kind, start, line, column))
    }

    fn make_at(&self, kind : TokenKind, start: usize, line: usize, column: usize) -> Token {
        Token {
            kind,
            span: crate::common::Span{
                start,
                end: self.current,
                line,
                column,
            },
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                if ch == '\n' {
                    self.line += 1;
                    self.column = 1;
                }
                self.advance();
            } else {
                break;
            }
        }
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.src[self.current..].chars().next()?;
        self.current += ch.len_utf8();
        self.column += 1;
        Some(ch)
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Option<char> {
        self.src[self.current..].chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        let mut iter = self.src[self.current..].chars();
        iter.next()?;
        iter.next()
    }
    
    fn is_at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    // ----- Lexeme Routines -----
    fn string(&mut self, _start: usize, line: usize, column: usize) -> Token {
        let start_content = self.current;
        while let Some(ch) = self.peek() {
            if ch == '"' { break; }
            if ch == '\n' { self.line += 1; self.column = 1; }
            self.advance();
        }

        // Intern the string
        let interned = self.pool.intern(&self.src[start_content..self.current]);

        let tok = self.make_at(
            TokenKind::StringLiteral(interned), 
            start_content, 
            line, 
            column
        );

        // Consume closing quote
        self.advance();

        tok
    } 

    fn number(&mut self, start: usize, line: usize, column: usize) -> Token {
        while self.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            self.advance();
        }

        if self.peek() == Some('.') && self.peek_next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            self.advance(); // consume the '.'
            while self.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                self.advance();
            }
        }

        // Get the value
        let lexeme = &self.src[start..self.current];

        if lexeme.contains('.') {
            let value: f64 = lexeme.parse().unwrap_or(0.0);

            return self.make_at(
                TokenKind::FloatLiteral(value), 
                start, 
                line, 
                column
            );
        }

        let value: i64 = lexeme.parse().unwrap_or(0);

        self.make_at(
            TokenKind::IntegerLiteral(value), 
            start, 
            line, 
            column
        )
    }

    fn identifier(&mut self, start: usize, line: usize, column: usize) -> Token {
        while self.peek().map(|c| is_ident_continue(c)).unwrap_or(false) {
            self.advance();
        }

        let lexeme = &self.src[start..self.current];

        let kind = match lexeme {
            // Keywords
            "let" => TokenKind::Let,
            "do" => TokenKind::Do,
            "end" => TokenKind::End,
            "if" => TokenKind::If,
            "elif" => TokenKind::Elif,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,

            // Operators
            "or" => TokenKind::Or,
            "and" => TokenKind::And,
            "not" => TokenKind::Not,

            // Types
            "int" => TokenKind::Integer,
            "float" => TokenKind::Float,
            "bool" => TokenKind::Boolean,
            "string" => TokenKind::String,

            // Literals
            "true" => TokenKind::True,
            "false" => TokenKind::False,

            // Default to identifier
            _ => {
                let sym = self.pool.intern(lexeme);
                TokenKind::Identifier(sym)
            },
        };

        self.make_at(kind, start, line, column)
    }

}

// Helpers
fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(src: &str) -> Vec<Token> {
        let mut pool = StringPool::new();
        let mut sink = DiagnosticsSink::new();
        let mut lexer = Lexer::new(src, &mut pool, &mut sink);
        lexer.lex()
    }

    fn kinds(tokens: &[Token]) -> Vec<TokenKind> {
        tokens.iter().map(|t| t.kind.clone()).collect()
    }

    #[test]
    fn test_lex_single_char_tokens() {
        let toks = lex("()+-*/%:");
        assert_eq!(
            kinds(&toks),
            vec![
                TokenKind::LeftParen,
                TokenKind::RightParen,
                TokenKind::Plus,
                TokenKind::Minus,
                TokenKind::Star,
                TokenKind::Slash,
                TokenKind::Modulus,
                TokenKind::Colon,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_lex_integer_literal() {
        let toks = lex("123");
        if let TokenKind::IntegerLiteral(v) = toks[0].kind {
            assert_eq!(v, 123);
        } else {
            panic!("Expected IntegerLiteral token");
        }
    }

    #[test]
    fn test_lex_float_literal() {
        let toks: Vec<Token> = lex("12.34");
        if let TokenKind::FloatLiteral(v) = toks[0].kind {
            assert_eq!(v, 12.34);
        } else {
            panic!("Expected FloatLiteral token");
        }
    }

    #[test]
    fn test_lex_identifier() {
        let mut pool = StringPool::new();
        let mut sink = DiagnosticsSink::new();
        let mut lexer = Lexer::new("hello_world123", &mut pool, &mut sink);
        let toks = lexer.lex();
        if let TokenKind::Identifier(symbol) = toks[0].kind {
            let resolved = pool.resolve(symbol);
            assert_eq!(resolved, "hello_world123");
        } else {
            panic!("Expected Identifier token");
        }
    }

    #[test]
    fn test_lex_keywords() {
        let toks = lex("do end if elif else while let int float bool string");
        assert_eq!(
            kinds(&toks),
            vec![
                TokenKind::Do,
                TokenKind::End,
                TokenKind::If,
                TokenKind::Elif,
                TokenKind::Else,
                TokenKind::While,
                TokenKind::Let,
                TokenKind::Integer,
                TokenKind::Float,
                TokenKind::Boolean,
                TokenKind::String,
                TokenKind::EOF,
            ]
        )
    }

    #[test]
    fn test_lex_string_literal() {
        let mut pool = StringPool::new();
        let mut sink = DiagnosticsSink::new();
        let mut lexer = Lexer::new("\"hello world\"", &mut pool, &mut sink);
        let toks = lexer.lex();
        if let TokenKind::StringLiteral(symbol) = toks[0].kind {
            let resolved = pool.resolve(symbol);
            assert_eq!(resolved, "hello world");
        } else {
            panic!("Expected StringLiteral token");
        }
    }

    #[test]
    fn test_lex_comparison_operators() {
        let toks = lex("== != <= >= < > = and or !");
        assert_eq!(
            kinds(&toks),
            vec![
                TokenKind::EQ,
                TokenKind::NEQ,
                TokenKind::LEQ,
                TokenKind::GEQ,
                TokenKind::LT,
                TokenKind::GT,
                TokenKind::Assign,
                TokenKind::And,
                TokenKind::Or,
                TokenKind::Error, // '!' alone is error
                TokenKind::EOF
            ]
        );
    }

    #[test]
    fn test_lex_whitespace_handling() {
        let mut pool = StringPool::new();
        let mut sink = DiagnosticsSink::new();
        let mut lexer = Lexer::new("  \n\t  let  \n  x  =  42  ", &mut pool, &mut sink);
        let toks = lexer.lex();
        assert_eq!(
            kinds(&toks),
            vec![
                TokenKind::Let,
                TokenKind::Identifier(pool.intern("x")),
                TokenKind::Assign,
                TokenKind::IntegerLiteral(42),
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_lex_only_whitespace() {
        let toks = lex("   \n\t  \n ");
        assert_eq!(kinds(&toks), vec![TokenKind::EOF]);
    }

    #[test]
    fn test_lex_unterminated_string() {
        let mut pool = StringPool::new();
        let mut sink = DiagnosticsSink::new();
        let mut lexer = Lexer::new("\"unterminated string", &mut pool, &mut sink);
        let toks = lexer.lex();
        if let TokenKind::StringLiteral(symbol) = toks[0].kind {
            let resolved = pool.resolve(symbol);
            assert_eq!(resolved, "unterminated string");
        } else {
            panic!("Expected StringLiteral token");
        }
    }

    #[test]
    fn test_lex_unknown_character() {
        let toks = lex("@");
        assert_eq!(toks[0].kind, TokenKind::Error);
        todo!("Check for correct diagnostic emission");
    }

    #[test]
    fn test_lex_complex_input() {
        let src = r#"
        let x = 42
        let y = 3.14
        let name = "Mimble"
        if x >= 10 and y < 5.0 do
            print(name)
        end
        "#;
        let mut pool = StringPool::new();
        let mut sink = DiagnosticsSink::new();
        let mut lexer = Lexer::new(src, &mut pool, &mut sink);
        let toks = lexer.lex();
        let expected_kinds = vec![
            TokenKind::Let,
            TokenKind::Identifier(pool.intern("x")),
            TokenKind::Assign,
            TokenKind::IntegerLiteral(42),
            TokenKind::Let,
            TokenKind::Identifier(pool.intern("y")),
            TokenKind::Assign,
            TokenKind::FloatLiteral(3.14),
            TokenKind::Let,
            TokenKind::Identifier(pool.intern("name")),
            TokenKind::Assign,
            TokenKind::StringLiteral(pool.intern("Mimble")),
            TokenKind::If,
            TokenKind::Identifier(pool.intern("x")),
            TokenKind::GEQ,
            TokenKind::IntegerLiteral(10),
            TokenKind::And,
            TokenKind::Identifier(pool.intern("y")),
            TokenKind::LT,
            TokenKind::FloatLiteral(5.0),
            TokenKind::Do,
            TokenKind::Identifier(pool.intern("print")),
            TokenKind::LeftParen,
            TokenKind::Identifier(pool.intern("name")),
            TokenKind::RightParen,
            TokenKind::End,
            TokenKind::EOF,
        ];
        assert_eq!(kinds(&toks), expected_kinds);
    }

    #[test]
    fn test_lex_empty_input() {
        let toks = lex("");
        assert_eq!(kinds(&toks), vec![TokenKind::EOF]);
    }
}