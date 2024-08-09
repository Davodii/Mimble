using VMProject.Exceptions;

namespace VMProject;

public abstract class ConstantValue(ValueType type) : Value(type)
{
    protected object _value;

    public ConstantValue(ValueType type, object value) : this(type)
    {
        _value = value;
    }
    
    public override object GetValue()
    {
        return _value;
    }
    
    public static bool IsNumber(Value val)
    {
        return val.GetValueType() == ValueType.Number;
    }

    public static bool IsBoolean(Value val)
    {
        return val.GetValueType() == ValueType.Boolean;
    }

    public static bool IsString(Value val)
    {
        return val.GetValueType() == ValueType.String;
    }
}