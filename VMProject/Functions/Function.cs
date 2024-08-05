namespace VMProject.Functions;

public abstract class Function(string identifier)
{
    public string Identifier { get; } = identifier;
    
    public int Arity { get; set; } = 0;


    public override string ToString()
    {
        return $"<Function {Identifier} ({Arity} params)>";
    }
}