using VMProject.Exceptions;

namespace VMProject;

public class Value(object value, ValueType type)
{
    public object GetValue()
    {
        return value;
    }

    public new ValueType GetValueType()
    {
        return type;
    }

    public override bool Equals(object? obj)
    {
        if (obj == null) return false;
        
        if (obj.GetType() != this.GetType()) return false;
        
        return value.Equals(((Value)obj).GetValue()) && type.Equals(((Value)obj).GetValueType());
    }

    public override string ToString()
    {
        return $"[ {type.ToString()} Value: {value}]";
    }
    
    // TODO: make these throw a conversion error that is picked up by the vm during runtime
    
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
    
    public int AsInteger()
    {
        if (!IsNumber(this) || (double)value % 1 != 0)
        {
            // Not a whole number
            throw new ConversionException(this, ValueType.Number);
        }
        return (int)((double)value);
    }

    public double AsNumber()
    {
        if (!IsNumber(this))
        {
            throw new ConversionException(this, ValueType.Number);
        }

        return (double)value;
    }

    public bool AsBoolean()
    {
        if (!IsBoolean(this))
        {
            throw new ConversionException(this, ValueType.Boolean);
        }

        return (bool)value;
    }

    public string AsString()
    {
        if (!IsString(this))
        {
            throw new ConversionException(this, ValueType.String);
        }

        return (string)value;
    }
}