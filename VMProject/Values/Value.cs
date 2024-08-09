using VMProject.Exceptions;

namespace VMProject;

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
        
        if (obj.GetType() != this.GetType()) return false;
        
        return GetValue() == ((Value)obj).GetValue() && type.Equals(((Value)obj).GetValueType());
    }

    public override string ToString()
    {
        return $"[ {type.ToString()} ]";
    }
    
    // TODO: make these throw a conversion error that is picked up by the vm during runtime
    
    
}