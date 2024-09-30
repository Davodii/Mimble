using Mimble.Functions.Lists;
using Mimble.Values;
using ValueType = Mimble.Values.ValueType;

namespace Mimble.Functions;

public static class GlobalScope
{
    // ! create the global environment for the interpreted code
    // ! include things like a functions (e.g. print, len, etc.)

    public static Environment CreateGlobal(GlobalFunctions globals)
    {
        Environment environment = new();
        
        if (globals.HasFlag(GlobalFunctions.IO))
        {
            // ! Print
            Native print = PrintFn.GetPrintFn();
            environment.Assign("print", new FunctionValue(ValueType.NativeFunction, print));
        }

        if (globals.HasFlag(GlobalFunctions.Lists))
        {
            // ! Length
            Native length = LengthFn.GetLengthFn();
            environment.Assign("length", new FunctionValue(ValueType.NativeFunction, length));

            // ! Append
            Native append = AppendFn.GetAppendFn();
            environment.Assign("append", new FunctionValue(ValueType.NativeFunction, append));

            // ! Pop
            Native pop = PopFn.GetPopFn();
            environment.Assign("pop", new FunctionValue(ValueType.NativeFunction, pop));
        }

        return environment;
    }
}