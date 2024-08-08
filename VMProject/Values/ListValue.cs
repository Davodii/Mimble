using System.Collections;

namespace VMProject;

public class ListValue
{
    private List<Value> _list = new List<Value>();
    
    public ListValue(int start, int end)
    {
        // Calculate increment from this
        int increment = (end - start) / Math.Abs(end - start);

        AddFromRange(start, end, increment);
    }

    public ListValue(int start, int end, int increment)
    {
        AddFromRange(start, end, increment);
    }

    public ListValue(Stack<Value> stack, int itemCount)
    {
        for (int i = 0; i < itemCount; i++)
        {
            _list.Insert(0,stack.Pop());
        }
    }
    
    private void AddFromRange(int start, int end, int increment)
    {
        for (int i = start; (start < end) ? i < end : i > end; i += increment)
        {
            _list.Add(new Value((double)i, ValueType.Number));
        }
    }

    public Value Get(int index)
    {
        return _list[index];
    }

    public int Count()
    {
        return _list.Count;
    }

    public void Append(Value val)
    {
        _list.Add(val);
    }

    public void RemoveAt(int index)
    {
        if (index == -1)
        {
            // Pop
            _list.RemoveAt(_list.Count - 1);
        }
        else
        {
            _list.RemoveAt(index);
        }
    }
    
    public override string ToString()
    {
        // Create a string of all the values in the list
        string values = "[";
        for (int i = 0; i < _list.Count; i++)
        {
            values += _list[i].GetValue();
            if (i != _list.Count - 1)
            {
                values += ',';
            }
        }

        values += ']';
        return values;
    }
}