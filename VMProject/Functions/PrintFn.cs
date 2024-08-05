namespace VMProject.Functions;

// TODO: make this a singleton?

public class PrintFn : Native
{
    private static readonly PrintFn _printFn = new PrintFn();

    private PrintFn() : base("print")
    {
        
    }

    public static PrintFn GetPrintFn()
    {
        return _printFn;
    }
    
    public override void Execute(Stack<Value> stack)
    {
        // Get the top value from the stack
        Value top = stack.Pop();
        
        // Print the value to the console
        Console.WriteLine(top.GetValue());
    }
}