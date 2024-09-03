using Mimble.Functions;

namespace Mimble.Values;

public class FunctionValue(ValueType type, Function function) : Value(type)
{
    public override Function GetValue()
    {
        return function;
    }

    public override string ToString()
    {
        return function.ToString();
    }
}