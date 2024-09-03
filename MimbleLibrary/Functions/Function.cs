namespace Mimble.Functions;

public abstract class Function(string identifier)
{
    public string identifier { get; } = identifier;
    
    public int arity { get; set; }


    public override string ToString()
    {
        return $"<Function {identifier} ({arity} params)>";
    }

    public abstract void PrintCode();
}