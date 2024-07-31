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
}