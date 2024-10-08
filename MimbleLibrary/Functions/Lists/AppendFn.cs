using Mimble.Exceptions;
using Mimble.Values;
using ValueType = Mimble.Values.ValueType;

namespace Mimble.Functions.Lists;

public class AppendFn : Native
{
    private static readonly AppendFn AAppendFn = new();

    public static AppendFn GetAppendFn()
    {
        return AAppendFn;
    }

    public AppendFn() : base("length")
    {
        arity = 2;
    }

    public override void Execute(VM vm)
    {
        Value toAdd = vm.Pop();
        Value listValue = vm.Pop();

        if (listValue.GetValueType() != ValueType.List)
            throw new RunTimeException(vm.CurrentLineNumber(), $"Expected a list but got '{listValue.GetValueType()}'.");

        ListValue list = (ListValue)listValue.GetValue();
        
        list.Append(toAdd);
        
        vm.Push(new NumberValue(list.Count()));
    }
}