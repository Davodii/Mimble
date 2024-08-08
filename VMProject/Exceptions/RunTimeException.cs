namespace VMProject;

public sealed class RunTimeException : Exception
{
    public readonly int Line;
    
    public RunTimeException(int line)
    {
        this.Line = line;
    }

    public RunTimeException(int line, string msg) : base(msg)
    {
        this.Line = line;
    }

    public RunTimeException(int line, string msg, Exception inner) : base(msg, inner)
    {
        this.Line = line;
    }
}