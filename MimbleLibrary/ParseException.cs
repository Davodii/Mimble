namespace Mimble;

public class ParseException : Exception
{
    public int Start;
    public int End;
    
    public ParseException(int start, int end)
    {
        Start = start;
        End = end;
    }

    public ParseException(int start, int end, string msg) : base(msg)
    {
        Start = start;
        End = end;
    }

    public ParseException(int start, int end, string msg, Exception inner) : base(msg, inner)
    {
        Start = start;
        End = end;
    }
}