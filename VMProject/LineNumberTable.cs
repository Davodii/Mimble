namespace VMProject;

public class LineNumberTable<T> where T : notnull
{
    private class Range(int start, int end)
    {
        public int End = end;

        public bool Contains(int i)
        {
            return start <= i && End >= i;
        }
    }

    private readonly Dictionary<T, Range> _table = new();

    public void AddIndexToLine(T line, int index)
    {
        // Check if the line is already contained
        if (_table.TryGetValue(line, out var value))
        {
            value.End = index;
        }
        else
        {
            _table.Add(line, new Range(index, index));
        }
    }

    public T GetLineInfo(int index)
    {
        // Reverse-search the table 
        return _table.First(pair => pair.Value.Contains(index)).Key;
    }
}