namespace Mimble.Functions;

public abstract class Native(string identifier) : Function(identifier)
{
    /// <summary>
    /// Execute the native function
    /// </summary>
    public abstract void Execute(VM vm);

    public override string ToString()
    {
        return $"<Native Function {identifier} ({arity} params)>";
    }

    public override void PrintCode()
    {
        Console.WriteLine(ToString());
        Console.WriteLine();
    }
}