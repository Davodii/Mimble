namespace Mimble.Values;

public class NullValue : Value
{
    private static readonly NullValue NNullValue = new();

    private NullValue() : base(ValueType.Null)
    {
    }

    public static NullValue GetNullValue()
    {
        return NNullValue;
    }
    
    public override object GetValue()
    {
        return null!;
    }

    public override string ToString()
    {
        return "null";
    }
}