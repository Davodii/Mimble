namespace Mimble.Values;

public abstract class Value
{
    private readonly ValueType _type;

    protected Value(ValueType type)
    {
        _type = type;
    }

    public abstract object GetValue();

    public ValueType GetValueType()
    {
        return _type;
    }

    public override bool Equals(object? obj)
    {
        if (obj == null) return false;
        
        if (obj.GetType() != GetType()) return false;
        
        return GetValue() == ((Value)obj).GetValue() && _type.Equals(((Value)obj).GetValueType());
    }

    public override string ToString()
    {
        return $"[ {_type.ToString()} ]";
    }

    public override int GetHashCode()
    {
        // ReSharper disable once BaseObjectGetHashCodeCallInGetHashCode
        return base.GetHashCode();
    }
}