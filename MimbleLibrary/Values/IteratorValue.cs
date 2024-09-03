namespace Mimble.Values;

public class IteratorValue(ListValue list) : Value(ValueType.Iterator)
{
    private int _current;

    public Value GetNext()
    {
        if (_current >= list.Count()) throw new IndexOutOfRangeException();
        return list.Get(_current++);
    }

    public override object GetValue()
    {
        return this;
    }
}