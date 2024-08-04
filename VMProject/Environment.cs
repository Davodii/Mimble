using System.Security.AccessControl;

namespace VMProject;

public class Environment(Environment enclosing = null!)
{
    private readonly Environment? _enclosing = enclosing;
    private readonly Dictionary<string, Value> _locals = new();

    private void Define(string name)
    {
        _locals.Add(name, new Value(null!, ValueType.Null));
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
    }

    public Value Get(string name)
    {
        if (_locals.TryGetValue(name, out var local))
        {
            return local;
        }

        if (_enclosing != null)
        {
            return _enclosing.Get(name);
        }
        
        throw new RunTimeException(0, $"No variable or function with identifier '{name}' is defined.");
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