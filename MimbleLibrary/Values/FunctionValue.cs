using Mimble.Functions;

namespace Mimble.Values;

public class FunctionValue : Value
{
    private readonly Function _function;

    public FunctionValue(ValueType type, Function function) : base(type)
    {
        _function = function;
    }

    public override Function GetValue()
    {
        return _function;
    }

    public override string ToString()
    {
        return _function.ToString();
    }
}