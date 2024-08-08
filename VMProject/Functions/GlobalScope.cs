using VMProject.Functions.Lists;
namespace VMProject.Functions;

public static class GlobalScope
{
    // ! create the global environment for the interpreted code
    // ! include things like a functions (e.g. print, len, etc.)

    private static readonly Environment Global;

    static GlobalScope()
    {
        // Initialize _global
        Global = new Environment();
        
        // ! create the global scope
        CreateGlobal();
    }

    private static void CreateGlobal()
    {
        // ! Print
        Native print = PrintFn.GetPrintFn();
        Global.Assign("print", new Value(print, ValueType.NativeFunction));
        
        // ! Length
        Native length = LengthFn.GetLengthFn();
        Global.Assign("length", new Value(length, ValueType.NativeFunction));
        
        // ! Append
        Native append = AppendFn.GetAppendFn();
        Global.Assign("append", new Value(append, ValueType.NativeFunction));
        
        // ! Pop
        Native pop = PopFn.GetPopFn();
        Global.Assign("pop", new Value(pop, ValueType.NativeFunction));
    }

    public static Environment GetGlobalScope()
    {
        return Global;
    }
}