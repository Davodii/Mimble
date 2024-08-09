using VMProject.Values;

namespace VMProject.Functions;

public class PrintFn : Native
{
    private static readonly PrintFn PPrintFn = new PrintFn();

    private PrintFn() : base("print")
    {
        Arity = 1;
    }

    public static PrintFn GetPrintFn()
    {
        return PPrintFn;
    }
    
    public override void Execute(VM vm)
    {
        // Get the top value from the stack
        Value top = vm.Pop();
        
        // Print the value to the console
        Console.WriteLine(top.GetValue());
        
        // Push the "value" of the function the stack
        vm.Push(new StringValue(ToString()));
    }
}