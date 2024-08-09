namespace VMProject;

public class IdentifierValue : ConstantValue
{
    private string _identifier;
    private Value _value;

    public IdentifierValue(string identifier) : base(ValueType.Identifier)
    {
        _identifier = identifier;
        _value = NullValue.GetNullValue();
    }

    public IdentifierValue(string identifier, Value value) : base(ValueType.Identifier)
    {
        _identifier = identifier;
        _value = value;
    }

    public string GetIdentifier()
    {
        return _identifier;
    }

    public void Assign(Value val)
    {
        _value = val;
    }

    public override object GetValue()
    {
        return _value;
    }

    public override string ToString()
    {
        return $"\"{_identifier}\": {_value}";
    }
}