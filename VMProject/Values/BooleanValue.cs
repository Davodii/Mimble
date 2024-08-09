namespace VMProject;

public class BooleanValue : ConstantValue
{
    public BooleanValue(bool value) : base(ValueType.Boolean)
    {
        _value = value;
    }
    
    public bool AsBoolean()
    {
        return (bool)_value;
    }
    
    public override string ToString()
    {
        return AsBoolean() ? "true" : "false";
    }
}