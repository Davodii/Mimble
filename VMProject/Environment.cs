namespace VMProject;

public class Environment
{
    private Environment? _enclosing;
    private Dictionary<string, Value> _locals;

    public void Define(string name)
    {
        _locals.Add(name, new Value(null, ValueType.Null));
    }

    public void Assign(string name, Value value)
    {
        if (_locals.ContainsKey(name))
        {
            _locals[name] = value;
        } 
        else if (_enclosing != null)
        {
            // Check if the variable exists in the enclosing environment
            _enclosing.Assign(name, value);
        }
        // TODO: error, no such variable exists
        return;
    }

    public Value Get(string name)
    {
        if (_locals.TryGetValue(name, out var local))
        {
            return local;
        }
        else if (_enclosing != null)
        {
            return _enclosing.Get(name);
        }
         
        // TODO: error, no such variable found
        return null;
    }
}