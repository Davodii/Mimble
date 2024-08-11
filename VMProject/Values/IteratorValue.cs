namespace VMProject.Values;

public class IteratorValue : Value
{
    private ListValue _list;
    private int _current = 0;
    
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
        throw new NotImplementedException();
    }
}