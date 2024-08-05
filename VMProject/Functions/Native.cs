namespace VMProject.Functions;

public abstract class Native(string identifier) : Function(identifier)
{
    /// <summary>
    /// Execute the native function
    /// </summary>
    public abstract void Execute(Stack<Value> stack);

    public override string ToString()
    {
        return $"<Native Function {Identifier} ({Arity} params)>";
    }

    public override void PrintCode()
    {
        Console.WriteLine(this.ToString());
        Console.WriteLine();
    }
}