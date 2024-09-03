namespace VMProject.Values;

public class BooleanValue : ConstantValue
{
    public BooleanValue(bool value) : base(ValueType.Boolean)
    {
        Value = value;
    }
    
    public bool AsBoolean()
    {
        return (bool)Value;
    }
    
    public override string ToString()
    {
        return AsBoolean() ? "true" : "false";
    }
}