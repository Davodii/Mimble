namespace VMProject;

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
}