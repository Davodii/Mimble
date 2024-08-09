namespace VMProject;

public class NullValue() : Value(ValueType.Null)
{
    private static readonly NullValue NNullValue = new();

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