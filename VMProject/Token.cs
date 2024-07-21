namespace VMProject;

public class Token(TokenType type, int start, int length, int line)
{
    public int GetStart()
    {
        return start;
    }

    public int GetLength()
    {
        return length;
    }

    public new TokenType GetType()
    {
        return type;
    }

    public int GetLine()
    {
        return line;
    }
}