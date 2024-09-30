namespace Mimble;

public class Token
{
    private readonly TokenType _type;
    private readonly int _start;
    private readonly string _value;
    private readonly int _line;

    public Token(TokenType type, int start, string value, int line)
    {
        _type = type;
        _start = start;
        _value = value;
        _line = line;
    }

    public int GetStart()
    {
        return _start;
    }

    public string GetValue()
    {
        return _value;
    }

    public new TokenType GetType()
    {
        return _type;
    }

    public int GetLine()
    {
        return _line;
    }

    public override string ToString()
    {
        return "line " + _line + ": " + _type + ": " + _value;
    }
}