using Mimble.Exceptions;
using Mimble.Values;

namespace Mimble;

public class Environment
{
    private readonly Environment? _enclosing;
    private readonly Dictionary<string, Value> _locals = new();

    public Environment(Environment enclosing = null!)
    {
        _enclosing = enclosing;
    }

    private void Define(string name)
    {
        _locals.Add(name, NullValue.GetNullValue());
    }

    public void Assign(string name, Value value)
    {
        if (_locals.ContainsKey(name))
        {
            _locals[name] = value;
        } 
        else if (_enclosing != null && _enclosing.Defined(name))
        {
            // Check if the variable exists in the enclosing environment
            _enclosing.Assign(name, value);
        }
        else
        {
            // No such variable exists, create it
            _locals.Add(name,value);
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
            return true;
        }
        return _enclosing != null && _enclosing.Defined(identifier);
    }

    public Dictionary<string, Value> GetLocals()
    {
        return _locals;
    }
    public Environment? GetEnclosing()
    {
        return _enclosing;
    }
    
}