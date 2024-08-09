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
        ConstantValue indexValue;
        ListValue listValue;
        try
        {
            indexValue = (ConstantValue)vm.Pop();
            listValue = (ListValue)vm.Pop();
        }
        catch (Exception e)
        {
            // print the error
            throw;
        }

        if (ConstantValue.IsNumber(indexValue))
            throw new RunTimeException(vm.CurrentLineNumber(), $"Expected a number but got '{indexValue.GetValueType()}'.");

        try
        {
            listValue.RemoveAt(((NumberValue)indexValue).AsInteger());
        }
        catch (Exception e)
        {
            switch (e)
            {
                case ConversionException exception:
                    throw new RunTimeException(vm.CurrentLineNumber(),
                        $"Expected a '{exception.Expected}' value, but got a '{exception.VValue.GetValueType()}' value.");
                case ArgumentOutOfRangeException:
                    throw new RunTimeException(vm.CurrentLineNumber(), $"Index ({((NumberValue)indexValue).AsInteger()}) was outside the range of the list ({listValue.Count()}).");
                default:
                    throw;
            }
        }

        vm.Push(new StringValue(ToString()));
    }
}