namespace Mimble.Functions;

public abstract class Function(string identifier)
{
    public string Identifier { get; } = identifier;
    
    public int Arity { get; set; }


    public override string ToString()
    {
        return $"<Function {Identifier} ({Arity} params)>";
    }

    public abstract void PrintCode();
}