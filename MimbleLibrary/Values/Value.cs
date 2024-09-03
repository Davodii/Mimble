namespace Mimble.Values;

public abstract class Value(ValueType type)
{
    public abstract object GetValue();

    public ValueType GetValueType()
    {
        return type;
    }

    public override bool Equals(object? obj)
    {
        if (obj == null) return false;
        
        if (obj.GetType() != GetType()) return false;
        
        return GetValue() == ((Value)obj).GetValue() && type.Equals(((Value)obj).GetValueType());
    }

    public override string ToString()
    {
        return $"[ {type.ToString()} ]";
    }

    public override int GetHashCode()
    {
        // ReSharper disable once BaseObjectGetHashCodeCallInGetHashCode
        return base.GetHashCode();
    }
}