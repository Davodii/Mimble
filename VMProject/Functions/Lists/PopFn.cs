using VMProject.Exceptions;

namespace VMProject.Functions.Lists;

public class PopFn : Native
{
    private static readonly PopFn PPopFn = new();

    public static PopFn GetPopFn()
    {
        return PPopFn;
    }

    private PopFn() : base("pop")
    {
        Arity = 2;
    }

    public override void Execute(VM vm)
    {
        Value indexValue = vm.Pop();
        Value listValue = vm.Pop();

        if (listValue.GetValueType() != ValueType.List)
            throw new RunTimeException(vm.CurrentLineNumber(), $"Expected a list but got '{listValue.GetValueType()}'.");

        ListValue list = (ListValue)listValue.GetValue();

        if (indexValue.GetValueType() != ValueType.Number)
            throw new RunTimeException(vm.CurrentLineNumber(), $"Expected a number but got '{indexValue.GetValueType()}'.");

        try
        {
            list.RemoveAt(indexValue.AsInteger());
        }
        catch (Exception e)
        {
            switch (e)
            {
                case ConversionException exception:
                    throw new RunTimeException(vm.CurrentLineNumber(),
                        $"Expected a '{exception.Expected}' value, but got a '{exception.VValue.GetValueType()}' value.");
                case ArgumentOutOfRangeException:
                    throw new RunTimeException(vm.CurrentLineNumber(), $"Index ({indexValue.AsInteger()}) was outside the range of the list ({list.Count()}).");
                default:
                    throw;
            }
        }

        vm.Push(new Value(ToString(), ValueType.String));
    }
}