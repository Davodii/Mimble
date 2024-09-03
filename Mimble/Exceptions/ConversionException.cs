using VMProject.Values;
using ValueType = VMProject.Values.ValueType;

namespace VMProject.Exceptions;

public sealed class ConversionException : Exception
{
    public readonly Value VValue;
    public readonly ValueType Expected;
    
    public ConversionException(Value value, ValueType expected)
    {
        VValue = value;
        Expected = expected;
    }

    public ConversionException(Value value, ValueType expected, string msg) : base(msg)
    {
        VValue = value;
        Expected = expected;
    }

    public ConversionException(Value value, ValueType expected, string msg, Exception inner) : base(msg, inner)
    {
        VValue = value;
        Expected = expected;
    }
}