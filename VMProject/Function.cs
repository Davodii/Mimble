namespace VMProject;

public class Function(string identifier, Chunk chunk)
{
    public string Identifier { get; } = identifier;
    public Chunk Chunk { get;  } = chunk;
    public int Arity { get; set; } = 0;

    public override string ToString()
    {
        return $"<Function {Identifier} ({Arity} params)>";
    }
}