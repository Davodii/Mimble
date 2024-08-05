namespace VMProject.Functions;

// TODO: make this a singleton?

public class PrintFn : Native
{
    private static readonly PrintFn PPrintFn = new PrintFn();

    private PrintFn() : base("print")
    {
        this.Arity = 1;
    }

    public static PrintFn GetPrintFn()
    {
        return PPrintFn;
    }
    
    public override void Execute(Stack<Value> stack)
    {
        // Get the top value from the stack
        Value top = stack.Pop();
        
        // Print the value to the console
        Console.WriteLine(top.GetValue());
    }
}