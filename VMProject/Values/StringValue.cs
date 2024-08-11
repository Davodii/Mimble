namespace VMProject.Values;

public class StringValue : ConstantValue
{
    public StringValue(string value) : base(ValueType.String)
    {
        Value = value;
    }
    
    public string AsString()
    {
        return (string)Value;
    }

    public override string ToString()
    {
        return AsString();
    }

    public override bool Equals(object? obj)
    {
        if (obj == null || obj.GetType() != GetType()) return false;
        return ((StringValue)obj).AsString() == AsString();
    }
}