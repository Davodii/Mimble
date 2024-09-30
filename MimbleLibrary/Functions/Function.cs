namespace Mimble.Functions;

public abstract class Function
{
    protected Function(string identifier)
    {
        this.identifier = identifier;
    }

    public string identifier { get; }
    
    public int arity { get; set; }


    public override string ToString()
    {
        return $"<Function {identifier} ({arity} params)>";
    }

    public abstract void PrintCode();
}