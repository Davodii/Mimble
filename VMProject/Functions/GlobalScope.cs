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
        
        // ! Len
    }

    public static Environment GetGlobalScope()
    {
        return Global;
    }
}