namespace VMProject;

public class Function(string identifier, Chunk chunk, int arity)
{
    private string _identifier = identifier;
    private Chunk _chunk = chunk;
    private int _arity = arity;

    public override string ToString()
    {
        return $"<Function {_identifier} ({_arity} params)>";
    }
}