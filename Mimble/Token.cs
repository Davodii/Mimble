namespace VMProject;

public class Token(TokenType type, int start, string value, int line)
{
    public int GetStart()
    {
        return start;
    }

    public string GetValue()
    {
        return value;
    }

    public new TokenType GetType()
    {
        return type;
    }

    public int GetLine()
    {
        return line;
    }

    public override string ToString()
    {
        return "line " + line + ": " + type + ": " + value;
    }
}