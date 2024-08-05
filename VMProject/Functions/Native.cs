namespace VMProject.Functions;

public abstract class Native(string identifier) : Function(identifier)
{
    /// <summary>
    /// Execute the native function
    /// </summary>
    public abstract void Execute(Stack<Value> stack);
}