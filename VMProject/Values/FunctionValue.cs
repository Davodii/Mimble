using VMProject.Functions;

namespace VMProject;

public class FunctionValue(ValueType type, Function function) : Value(type)
{
    public override Function GetValue()
    {
        return function;
    }
}