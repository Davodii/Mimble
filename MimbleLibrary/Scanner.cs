using Mimble.Exceptions;

namespace Mimble;

public class Scanner
{
    /*
     * Take source code and produce Tokens on demand
     */
    private string _source;
    private int _start;
    private int _current;
    private int _lines = 1;

    public Scanner()
    {
    }

    public void SetSource(string source)
    {
        _source = source;
    }
    
    private char Advance()
    {
        _current++;
        return _source[_current - 1];
    }

    private char Peek()
    {
        return _source[_current];
    }

    private char PeekNext()
    {
        return IsAtEnd() ? '\0' : _source[_current + 1];
    }

    private bool IsAtEnd()
    {
        return _current >= _source.Length || _start >= _source.Length;
    }

    private bool Match(char c)
    {
        if (IsAtEnd()) return false;
        if (_source[_current] != c) return false;

        Advance();
        return true;
    }

    private static bool IsAlpha(char c)
    {
        return c is >= 'a' and <= 'z' or >= 'A' and <= 'Z' or '_';
    }

    private static bool IsDigit(char c)
    {
        return c is >= '0' and <= '9';
    }

    private TokenType CheckKeyword(int offset, string rest, TokenType type)
    {
        // Check from _start for rest to see if the keyword matches
        if (_current - _start - offset != rest.Length || 
            _start + rest.Length + offset > _source.Length) 
            return TokenType.Identifier;
        
        return rest.Where((t, i) => _source[_start + i + offset] != t).Any() ? TokenType.Identifier : type;
    }

    private TokenType CheckTwoKeywords(int offset, string keyword1, string keyword2, TokenType type1, TokenType type2)
    {
        if (CheckKeyword(offset, keyword1, type1) == type1)
        {
            return type1;
        }

        return CheckKeyword(offset, keyword2, type2) == type2 ? type2 : TokenType.Identifier;
    }
    
    private TokenType IdentifierType()
    {
        // Match the token stored between _start and _current to a TokenType
        switch (_source[_start])
        {
            case 'a': return CheckKeyword(1, "nd", TokenType.And);
            case 'b': return CheckKeyword(1, "reak", TokenType.Break);
            case 'c': return CheckKeyword(1, "ontinue", TokenType.Continue);
            case 'd':
                // does
                // do
                return CheckTwoKeywords(1, "oes", "o", TokenType.Does, TokenType.Do);
            case 'e':
                // end 
                // else
                // elif
                if (CheckKeyword(1, "nd", TokenType.End) == TokenType.End) return TokenType.End;
                if (CheckKeyword(2, "se", TokenType.Else) == TokenType.Else) return TokenType.Else;
                return CheckKeyword(2, "if", TokenType.Elif);
            case 'f':
                // for
                // function
                // false
                TokenType firstCheck = CheckTwoKeywords(1, "or", "alse", TokenType.For, TokenType.False);
                return firstCheck == TokenType.Identifier ? CheckKeyword(1, "unction", TokenType.Function) : firstCheck;
            case 'i':
                // if
                // in
                return CheckTwoKeywords(1, "n", "f", TokenType.In, TokenType.If);
            case 'n':
                // null
                // not
                return CheckTwoKeywords(1, "ull", "ot", TokenType.Null, TokenType.Not);
            case 'o':
                return CheckKeyword(1, "r", TokenType.Or);
            case 'r':
                return CheckKeyword(1, "eturn", TokenType.Return);
            case 't':
                return CheckKeyword(1, "rue", TokenType.True);
            case 'w':
                return CheckKeyword(1, "hile", TokenType.While);
        }
        
        // Unidentified token
        return TokenType.Identifier;
    }

    
    private Token Identifier()
    {
        // Keep advancing until a non-alpha or digit is found
        while (!IsAtEnd() && IsAlpha(Peek()) || IsDigit(Peek())) Advance();

        // Current is on the next token
        return MakeToken(IdentifierType());
    }

    private Token Number()
    {
        while (!IsAtEnd() && IsDigit(Peek())) Advance();
        
        // Check for fractional part
        if (Peek() != '.' || !IsDigit(PeekNext())) return MakeToken(TokenType.Number);
        Advance();

        // Continue reading until no more digits are found
        while (IsDigit(Peek())) Advance();

        return MakeToken(TokenType.Number);
    }

    private Token SString()
    {
        // Still on "
        _start = _current;
        
        while (Peek() != '"' && !IsAtEnd())
        {
            if (Peek() == '\n') _lines++;
            Advance();
        }
        
        if (IsAtEnd()) throw new CompileTimeException(MakeToken(TokenType.Error),"Unterminated string.");
        
        Token token =  MakeToken(TokenType.String);
        // Consume the closing quote
        Advance();

        return token;
    }
    
    private void SkipWhitespace()
    {
        // Continue until a non whitespace character is found
        for (;;)
        {
            switch (Peek())
            {
                case '#':
                    while (Peek() != '\n' && !IsAtEnd()) Advance();
                    _lines++;
                    Advance();
                    break;
                case ' ':
                case '\r':
                case '\t':
                    Advance();
                    break;
                default:
                    return;
            }
        }
    }
    
    private Token MakeToken(TokenType type)
    {
        if (type == TokenType.Eof)
        {
            return new Token(TokenType.Eof, _start, "", _lines);
        }
        
        string value = _source.Substring(_start, _current - _start);
        Token token = new Token(type, _start, value, _lines);
        _start = _current;
        return token;
    }
    
    
    public Token ScanToken()
    {
        if (IsAtEnd()) return MakeToken(TokenType.Eof);
        
        // Skip whitespace
        SkipWhitespace();
        _start = _current;
        
        
        char c = Advance();
        
        
        // Check literals and identifiers
        if (IsAlpha(c)) return Identifier();
        if (IsDigit(c)) return Number();
        
        // Check other characters
        switch (c)
        {
            case '(': return MakeToken(TokenType.LeftParen);
            case ')': return MakeToken(TokenType.RightParen);
            case '[': return MakeToken(TokenType.SqLeftBrace);
            case ']': return MakeToken(TokenType.SqRightBrace);
            case ',': return MakeToken(TokenType.Comma);
            case '-': return MakeToken(TokenType.Minus);
            case '+': return MakeToken(TokenType.Plus);
            case '/': return MakeToken(TokenType.Slash);
            case '*': return MakeToken(TokenType.Star);
            case ':': return MakeToken(TokenType.Colon);
            case '\n':
                _lines++;
                return MakeToken(TokenType.Eol);
            case '=':
                return MakeToken(Match('=') ? TokenType.EqualEqual : TokenType.Equal);
            case '>':
                return MakeToken(Match('=') ? TokenType.GreaterEqual : TokenType.Greater);
            case '<':
                return MakeToken(Match('=') ? TokenType.LessEqual : TokenType.Less);
            case '"':
                return SString();
            case '.':
                if (Match('.')) return MakeToken(TokenType.DoubleDot);
                break;
        }
        
        throw new ParseException(_start, _current, "No valid token could be matched.");
    }
}