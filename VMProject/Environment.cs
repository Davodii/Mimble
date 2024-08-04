using System.Security.AccessControl;

namespace VMProject;

public class Environment(Environment enclosing = null)
{
    private Environment? _enclosing = enclosing;
    private Dictionary<string, Value> _locals;

    private void Define(string name)
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
        else
        {
            // No such variable exists, create it
            Define(name);
            _locals[name] = value;
        }
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

    public bool Defined(string identifier)
    {
        if (_locals.ContainsKey(identifier))
        {
            return false;
        }
        else if (_enclosing == null)
        {
            return false;
        }

        return _enclosing.Defined(identifier);
    }

    public Environment? GetEnclosing()
    {
        return _enclosing;
    }
}