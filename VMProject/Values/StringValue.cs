namespace VMProject.Values;

public class StringValue : ConstantValue
{
    public StringValue(string value) : base(ValueType.String)
    {
        _value = value;
    }
    
    public string AsString()
    {
        return (string)_value;
    }

    public override string ToString()
    {
        return AsString();
    }

    public override bool Equals(object? obj)
    {
        if (obj.GetType() != GetType()) return false;
        return ((StringValue)obj).AsString() == AsString();
    }
}