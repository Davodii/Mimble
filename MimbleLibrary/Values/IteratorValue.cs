namespace Mimble.Values;

public class IteratorValue : Value
{
    private int _current;
    private readonly ListValue _list;

    public IteratorValue(ListValue list) : base(ValueType.Iterator)
    {
        _list = list;
    }

    public Value GetNext()
    {
        if (_current >= _list.Count()) throw new IndexOutOfRangeException();
        return _list.Get(_current++);
    }

    public override object GetValue()
    {
        return this;
    }
}