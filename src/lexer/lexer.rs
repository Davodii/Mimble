use super::{Token, TokenKind};
use super::{LexerError, LexerErrorKind};
use crate::{Location};
pub struct Lexer<'a> {
    src: &'a str,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    fn error(&self, kind: LexerErrorKind) -> LexerError {
        LexerError {
            kind,
            location: Location {
                line: self.line,
                column: self.column,
            },
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;
            if let Some(token) = self.scan_token() {
                tokens.push(token);
            }
        }

        tokens.push(self.make(TokenKind::EOF));
        
        Ok(tokens)
    }

    fn scan_token(&mut self) -> Option<Token> {
        let c = self.advance()?;

        let kind = match c {
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            // ',' => TokenKind::Comma,
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

            '"' => return Some(self.string()),

            c if c.is_ascii_digit() => {
                return Some(self.number());
            },

            c if is_ident_start(c) => {
                return Some(self.identifier());
            },

            c if c.is_whitespace() => {
                if c == '\n' {
                    self.line += 1;
                    self.column = 1;
                } return None
            },

            _ => {
                // Unknown character
                TokenKind::Error
            },
        };

        Some(self.make(kind))
    }

    fn make(&self, kind : TokenKind) -> Token {
        Token {
            kind,
            lexeme: self.src[self.start..self.current].to_string(),
            loc: Location{
                line: self.line,
                column: self.column,
            },
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
    fn string(&mut self) -> Token {
        while let Some(ch) = self.peek() {
            if ch == '"' { break; }
            if ch == '\n' { self.line += 1; self.column = 1; }
            self.advance();
        }

        // Consume closing quote
        self.advance();

        let tok = self.make(TokenKind::StringLiteral);

        tok
    } 

    fn number(&mut self) -> Token {
        while self.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            self.advance();
        }

        if self.peek() == Some('.') && self.peek_next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            self.advance(); // consume the '.'
            while self.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                self.advance();
            }
        }

        self.make(TokenKind::NumericLiteral)
    }

    fn identifier(&mut self) -> Token {
        while self.peek().map(|c| is_ident_continue(c)).unwrap_or(false) {
            self.advance();
        }

        let lexeme = &self.src[self.start..self.current];

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
            "true" => TokenKind::TrueLiteral,
            "false" => TokenKind::FalseLiteral,

            // Default to identifier
            _ => TokenKind::Identifier,
        };

        self.make(kind)
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

    fn lex(src: &str) -> Result<Vec<Token>, LexerError> {
        let mut lexer = Lexer::new(src);
        lexer.lex()
    }

    fn kinds(tokens: &[Token]) -> Vec<TokenKind> {
        tokens.iter().map(|t| t.kind.clone()).collect()
    }

    #[test]
    fn test_lex_single_char_tokens() {
        let toks = lex("()+-*/%:").unwrap();
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
        let toks = lex("123").unwrap();
        assert_eq!(toks[0].kind, TokenKind::NumericLiteral);
        assert_eq!(toks[0].lexeme, "123");
    }

    #[test]
    fn test_lex_float_literal() {
        let toks: Vec<Token> = lex("12.34").unwrap();
        assert_eq!(toks[0].kind, TokenKind::NumericLiteral);
        assert_eq!(toks[0].lexeme, "12.34");
    }

    #[test]
    fn test_lex_identifier() {
        let toks = lex("hello_world123").unwrap();
        assert_eq!(toks[0].kind, TokenKind::Identifier);
        assert_eq!(toks[0].lexeme, "hello_world123");
    }

    #[test]
    fn test_lex_keywords() {
        let toks = lex("do end if elif else while let int float bool string").unwrap();
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
        let toks = lex("\"hello world\"").unwrap();
        assert_eq!(toks[0].kind, TokenKind::StringLiteral);
    }

    #[test]
    fn test_lex_comparison_operators() {
        let toks = lex("== != <= >= < > = and or !").unwrap();
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
        let toks = lex("  \n\t  let  \n  x  =  42  ").unwrap();
        assert_eq!(
            kinds(&toks),
            vec![
                TokenKind::Let,
                TokenKind::Identifier,
                TokenKind::Assign,
                TokenKind::NumericLiteral,
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_lex_only_whitespace() {
        let toks = lex("   \n\t  \n ").unwrap();
        assert_eq!(kinds(&toks), vec![TokenKind::EOF]);
    }

    #[test]
    fn test_lex_unterminated_string() {
        let result = lex("\"unterminated string");
        assert!(result.is_ok()); // Lexer does not currently handle errors
        let toks = result.unwrap();
        assert_eq!(toks[0].kind, TokenKind::StringLiteral);
    }

    #[test]
    fn test_lex_unknown_character() {
        let toks = lex("@").unwrap();
        assert_eq!(toks[0].kind, TokenKind::Error);
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
        let toks = lex(src).unwrap();
        let expected_kinds = vec![
            TokenKind::Let,
            TokenKind::Identifier,
            TokenKind::Assign,
            TokenKind::NumericLiteral,
            TokenKind::Let,
            TokenKind::Identifier,
            TokenKind::Assign,
            TokenKind::NumericLiteral,
            TokenKind::Let,
            TokenKind::Identifier,
            TokenKind::Assign,
            TokenKind::StringLiteral,
            TokenKind::If,
            TokenKind::Identifier,
            TokenKind::GEQ,
            TokenKind::NumericLiteral,
            TokenKind::And,
            TokenKind::Identifier,
            TokenKind::LT,
            TokenKind::NumericLiteral,
            TokenKind::Do,
            TokenKind::Identifier,
            TokenKind::LeftParen,
            TokenKind::Identifier,
            TokenKind::RightParen,
            TokenKind::End,
            TokenKind::EOF,
        ];
        assert_eq!(kinds(&toks), expected_kinds);
    }

    #[test]
    fn test_lex_empty_input() {
        let toks = lex("").unwrap();
        assert_eq!(kinds(&toks), vec![TokenKind::EOF]);
    }
}