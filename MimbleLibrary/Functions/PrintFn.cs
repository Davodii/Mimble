using Mimble.Values;

namespace Mimble.Functions;

public class PrintFn : Native
{
    private static readonly PrintFn PPrintFn = new();

    private PrintFn() : base("print")
    {
        arity = 1;
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
        Console.WriteLine(top.ToString());
        
        // Push the "value" of the function the stack
        vm.Push(new StringValue(ToString()));
    }
}