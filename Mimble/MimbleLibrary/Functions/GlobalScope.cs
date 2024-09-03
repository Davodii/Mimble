using Mimble.Functions.Lists;
using Mimble.Values;
using ValueType = Mimble.Values.ValueType;

namespace Mimble.Functions;

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
        Global.Assign("print", new FunctionValue(ValueType.NativeFunction, print));
        
        // ! Length
        Native length = LengthFn.GetLengthFn();
        Global.Assign("length", new FunctionValue(ValueType.NativeFunction, length));
        
        // ! Append
        Native append = AppendFn.GetAppendFn();
        Global.Assign("append", new FunctionValue(ValueType.NativeFunction, append));
        
        // ! Pop
        Native pop = PopFn.GetPopFn();
        Global.Assign("pop", new FunctionValue(ValueType.NativeFunction, pop));
    }

    public static Environment GetGlobalScope()
    {
        return Global;
    }
}