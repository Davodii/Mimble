namespace Mimble;

public class ParseException : Exception
{
    public int Start;
    public int End;
    
    public ParseException(int start, int end)
    {
        this.Start = start;
        this.End = end;
    }

    public ParseException(int start, int end, string msg) : base(msg)
    {
        this.Start = start;
        this.End = end;
    }

    public ParseException(int start, int end, string msg, Exception inner) : base(msg, inner)
    {
        this.Start = start;
        this.End = end;
    }
}