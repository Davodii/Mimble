namespace VMProject;

public class Scanner
{
    /*
     * Take source code and produce Tokens on demand
     */

    private readonly string _source;
    private int _start;
    private int _current;
    private int _lines;

    public Scanner(string source)
    {
        _source = source;
        _start = 0;
        _current = 0;
        _lines = 1;
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
        if (IsAtEnd()) return '\0';
        return _source[_current + 1];
    }

    private bool IsAtEnd()
    {
        if (_current >= _source.Length) return true;
        
        // Check if the end of a string is encountered
        return _source[_current] == '\0';
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
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_');
    }

    private static bool IsDigit(char c)
    {
        return c is >= '0' and <= '9';
    }

    private TokenType CheckKeyword(int offset, string rest, TokenType type)
    {
        //TODO: check this function actually works
        
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

        if (CheckKeyword(offset, keyword2, type2) == type2)
        {
            return type2;
        }

        return TokenType.Identifier;
    }
    
    private TokenType IdentifierType()
    {
        // Match the token stored between _start and _current to a TokenType

        //TODO: implement (and also figure out all the keywords)
        //TODO: check this actually works for multiple keywords
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
                return CheckTwoKeywords(1, "nd", "lse", TokenType.End, TokenType.Else);
            case 'f':
                // for
                // function
                // false
                TokenType firstCheck = CheckTwoKeywords(1, "or", "alse", TokenType.For, TokenType.False);
                return firstCheck != TokenType.Identifier ? CheckKeyword(1, "unction", TokenType.Function) : firstCheck;
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
        while (IsAlpha(Peek()) || IsDigit(Peek())) Advance();

        
        return MakeToken(IdentifierType());
    }

    private Token Number()
    {
        while (IsDigit(Peek())) Advance();
        
        // Check for fractional part
        if (Peek() == '.' && IsDigit(PeekNext()))
        {
            Advance();

            // Continue reading until no more digits are found
            while (IsDigit(Peek())) Advance();
        }

        return MakeToken(TokenType.Number);
    }

    private Token SString()
    {
        // Still on "
        Advance();
        _start = _current;
        
        while (Peek() != '"' && !IsAtEnd())
        {
            if (Peek() == '\n') _lines++;
            Advance();
        }

        //TODO: parse error, unterminated string.
        if (IsAtEnd()) return null;
        
        Token token =  MakeToken(TokenType.String);
        // Closing quote
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
                //TODO: handle comments maybe?
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
        // Make everything inside _previous to _current a token with the correct TokenType
        string value = _source.Substring(_start, _current - _start);
        
        Token token = new Token(type, _start, value, _lines);
        _current++;
        _start = _current;

        return token;
    }
    
    
    public Token ScanToken()
    {
        if (IsAtEnd()) return MakeToken(TokenType.Eof);
        
        // Skip whitespace
        SkipWhitespace();
        _start = _current;


        char c = _source[_current];
        
        
        // Check literals and identifiers
        if (IsAlpha(c)) return Identifier();
        if (IsDigit(c)) return Number();

        // TODO: check this is necessary
        Advance();
        
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
            case '!':
                return MakeToken(Match('=') ? TokenType.BangEqual : TokenType.Bang);
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
        
        // Unexpected token if here
        //TODO: throw a parse error, no valid token was found
        return new Token(TokenType.Error, _start, "", _lines);
    }
}