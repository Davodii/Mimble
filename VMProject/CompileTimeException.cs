namespace VMProject;

public class CompileTimeException : Exception
{
    public Token Token;
    
    public CompileTimeException(Token token)
    {
        this.Token = token;
    }

    public CompileTimeException(Token token, string msg) : base(msg)
    {
        this.Token = token;
    }

    public CompileTimeException(Token token, string msg, Exception inner) : base(msg, inner)
    {
        this.Token = token;
    }
}