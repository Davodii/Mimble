namespace Mimble.Values;

public abstract class ConstantValue : Value
{
    protected object Value = null!;

    protected ConstantValue(ValueType type, object value) : this(type)
    {
        Value = value;
    }

    protected ConstantValue(ValueType type) : base(type)
    {
    }

    public override object GetValue()
    {
        return Value;
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