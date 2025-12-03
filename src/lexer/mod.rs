// Lexer for the language
pub mod token;

pub use token::{Token, TokenKind, Span};

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

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;
            if let Some(token) = self.scan_token() {
                tokens.push(token);
            }
        }

        tokens.push(Token {
            kind: TokenKind::EOF,
            span: Span { start: self.current, end: self.current, },
            line: self.line,
            column: self.column,
        });
        
        tokens
    }

    pub fn lexeme(&self, token: &Token) -> &'a str {
        &self.src[token.span.start..token.span.end]
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
            span: Span {
                start: self.start,
                end: self.current,
            },
            line: self.line,
            column: self.column,
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

        self.make(TokenKind::StringLiteral)
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
            "do" => TokenKind::Do,
            "end" => TokenKind::End,
            "if" => TokenKind::If,
            "elif" => TokenKind::Elif,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "or" => TokenKind::Or,
            "and" => TokenKind::And,
            "not" => TokenKind::Not,
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

    fn lex(src: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(src);
        lexer.lex()
    }

    fn kinds(tokens: &[Token]) -> Vec<TokenKind> {
        tokens.iter().map(|t| t.kind.clone()).collect()
    }

    #[test]
    fn test_single_char_tokens() {
        let toks = lex("()+-*/%");
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
                TokenKind::EOF,
            ]
        );
    }

    #[test]
    fn test_number_literal() {
        let toks = lex("123");
        assert_eq!(toks[0].kind, TokenKind::NumericLiteral);
        assert_eq!(toks[0].span.start, 0);
        assert_eq!(toks[0].span.end, 3);
    }

    #[test]
    fn test_float_literal() {
        let toks: Vec<Token> = lex("12.34");
        assert_eq!(toks[0].kind, TokenKind::NumericLiteral);
        assert_eq!(toks[0].span.start, 0);
        assert_eq!(toks[0].span.end, 5);
    }

    #[test]
    fn test_identifier() {
        let toks = lex("hello_world123");
        assert_eq!(toks[0].kind, TokenKind::Identifier);
        assert_eq!(toks[0].span.start, 0);
        assert_eq!(toks[0].span.end, 14);
    }

    #[test]
    fn test_keywords() {
        let toks = lex("do end if elif else while");
        assert_eq!(
            kinds(&toks),
            vec![
                TokenKind::Do,
                TokenKind::End,
                TokenKind::If,
                TokenKind::Elif,
                TokenKind::Else,
                TokenKind::While,
                TokenKind::EOF,
            ]
        )
    }

    #[test]
    fn test_string_literal() {
        let toks = lex("\"hello world\"");
        assert_eq!(toks[0].kind, TokenKind::StringLiteral);
    }

    #[test]
    fn test_comparisons() {
        let toks = lex("== != <= >= < > = !");
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
                TokenKind::Error, // '!' alone is error
                TokenKind::EOF
            ]
        );
    }

    #[test]
    fn test_lexeme() {
        let mut lexer = Lexer::new("identifier");
        let toks = lexer.lex();
        assert_eq!(
            lexer.lexeme(&toks[0]),
            "identifier"
        );
    }
}