namespace VMProject;

public class Value(object value, ValueType type)
{
    public object GetValue()
    {
        return value;
    }

    public new ValueType GetType()
    {
        return type;
    }
}