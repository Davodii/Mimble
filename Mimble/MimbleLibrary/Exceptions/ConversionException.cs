using Mimble.Values;
using Values_ValueType = Mimble.Values.ValueType;
using ValueType = Mimble.Values.ValueType;

namespace Mimble.Exceptions;

public sealed class ConversionException : Exception
{
    public readonly Value VValue;
    public readonly Values_ValueType Expected;
    
    public ConversionException(Value value, Values_ValueType expected)
    {
        VValue = value;
        Expected = expected;
    }

    public ConversionException(Value value, Values_ValueType expected, string msg) : base(msg)
    {
        VValue = value;
        Expected = expected;
    }

    public ConversionException(Value value, Values_ValueType expected, string msg, Exception inner) : base(msg, inner)
    {
        VValue = value;
        Expected = expected;
    }
}