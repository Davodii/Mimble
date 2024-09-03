using Mimble.Exceptions;
using Mimble.Values;
using ValueType = Mimble.Values.ValueType;

namespace Mimble.Functions.Lists;

public class LengthFn : Native
{
    private static readonly LengthFn LLengthFn = new();

    public static LengthFn GetLengthFn()
    {
        return LLengthFn;
    }

    public LengthFn() : base("length")
    {
        Arity = 1;
    }

    public override void Execute(VM vm)
    {
        Value value = vm.Pop();

        if (value.GetValueType() != ValueType.List)
            throw new RunTimeException(vm.CurrentLineNumber(), $"Expected a list but got '{value.GetValueType()}'.");

        ListValue list = (ListValue)value.GetValue();
        vm.Push(new NumberValue(list.Count()));
    }
}