using System.Runtime.Intrinsics.X86;

namespace VMProject;

public class VM
{
    /* 
     * Perform interpreting for any passed in chunks of code.
     */

    private int _ip; // Instruction pointer to current bytecode instruction
    private Chunk _current;
    private Stack<Value> _valueStack;

    private Dictionary<Value, Value> _globals;

    public VM()
    {
        _valueStack = new Stack<Value>();
        _globals = new Dictionary<Value, Value>();
        _ip = 0;
        _current = null;
    }

    private void IncrementIP()
    {
        _ip++;
    }

    private void SetIP(int ip)
    {
        // Useful for jumps
        _ip = ip;
    }

    private void Push(Value value)
    {
        _valueStack.Push(value);
    }

    private Value Pop()
    {
        return _valueStack.Pop();
    }
    
    
    #region Utility Functions

    private double AsNumber(Value val)
    {
        // TODO: check the number is not null
        if (val.GetValueType() != ValueType.Number)
        {
            //TODO: error
            return -0;
        }

        return (double)val.GetValue();
    }

    private string AsString(Value val)
    {
        if (val.GetValueType() != ValueType.String)
        {
            //TODO: error
            return "\0";
        }

        return (string)val.GetValue();
    }

    private string ToString(Value val)
    {
        // Convert the value to a string 
        switch (val.GetValueType())
        {
            case ValueType.Null:
                return "null";
            case ValueType.Boolean:
                if ((bool)val.GetValue()) return "true";
                return "false";
            case ValueType.Number:
                return AsNumber(val).ToString();
            case ValueType.String:
                return AsString(val);
        }
        
        // Should never reach here
        return "ERROR";
    }
    
    #endregion
    
    #region Binary Operations

    private bool AreNumbers(Value val1, Value val2)
    {
        return val1.GetValueType() == ValueType.Number && val2.GetValueType() == ValueType.Number;
    }
    
    private void Add()
    {
        Value val1 = Pop();
        Value val2 = Pop();

        if (val1.GetValueType() == ValueType.String || val2.GetValueType() == ValueType.String)
        {
            // Perform string concatenation
            string result = ToString(val2) + ToString(val1);
            
            // push the result back onto the stack
            Push(new Value(result, ValueType.String));
        }
        else if (AreNumbers(val1, val2))
        {
            // Both values are numbers
            double result = AsNumber(val1) + AsNumber(val2);
            Push(new Value(result, ValueType.Number));
        }
        else
        {
            // ! invalid state
            //TODO: throw error
            return;
        }
    }
    
    private void Subtract()
    {
        Value val1 = Pop();
        Value val2 = Pop();

        if (!AreNumbers(val1, val2))
        {
            // ! Error
            return;
        }

        // val1 is on top of val2 so the actual order is
        // val2 - val1
        double result = AsNumber(val2) - AsNumber(val1);
        
        Push(new Value(result, ValueType.Number));
    }

    private void Multiply()
    {
        Value val1 = Pop();
        Value val2 = Pop();
        
        if (!AreNumbers(val1, val2))
        {
            // ! Error
            return;
        }

        double result = AsNumber(val1) * AsNumber(val2);
        Push(new Value(result, ValueType.Number));
    }

    private void Divide()
    {
        Value val1 = Pop();
        Value val2 = Pop();

        if (!AreNumbers(val1, val2))
        {
            // ! Error
        }

        double result = AsNumber(val2) / AsNumber(val1);
        Push(new Value(result, ValueType.Number));
    }
    
    #endregion
    
    #region Unary Operations

    private void Negate()
    {
        Value val = Pop();
        if (val.GetValueType() != ValueType.Number)
        {
            // ! Error
            return;
        }

        double result = -AsNumber(val);
        Push(new Value(result, ValueType.Number));
    }
    #endregion

    private void Interpret()
    {
        for (;;)
        {
            // Given a current chunk, interpret the code
            // Read the current instruction
            Instruction instruction = (Instruction)_current.GetByte(_ip);

            switch (instruction)
            {
                case Instruction.Pop:
                    Pop();
                    break;
                case Instruction.Null:
                    Push(new Value(null, ValueType.Null));
                    break;
                case Instruction.False:
                    Push(new Value(false, ValueType.Boolean));
                    break;
                case Instruction.True:
                    Push(new Value(true, ValueType.Boolean));
                    break;
                case Instruction.LoadConstant:
                    // Increment the _ip
                    IncrementIP();
                    Value constant = _current.GetLocal(_current.GetByte(_ip));
                    Push(constant);
                    break;
                case Instruction.Add:
                    Add();
                    break;
                case Instruction.Subtract:
                    Subtract();
                    break;
                case Instruction.Multiply:
                    Multiply();
                    break;
                case Instruction.Divide:
                    Divide();
                    break;
                case Instruction.Negate:
                    Negate();
                    break;
                case Instruction.Equal:
                    // Handle variable definition and / or initialization
                    break;
                case Instruction.Greater:
                    break;
                case Instruction.Less:
                    break;
                case Instruction.Not:
                    break;
                case Instruction.Jump:
                    break;
                case Instruction.JumpIfFalse:
                    break;
                case Instruction.Loop:
                    break;
                case Instruction.Call:
                    break;
                case Instruction.Return:
                    break;
                case Instruction.And:
                    break;
                case Instruction.Or:
                    break;
                case Instruction.StoreVar:
                    // Pop the top of the stack and store it into the variable given by the second 
                    Value val = Pop();
                    IncrementIP();
                    
                    // IP now points to the index of the variable name
                    byte index = _current.GetByte(_ip);
                    Value varIdentifier = _current.GetLocal(index);
                    
                    // Store the value inside the identifier in the list
                    _globals.Add(varIdentifier, val);
                    break;
                case Instruction.LoadVar:
                    break;
                case Instruction.End:
                    return;
                default:
                    throw new ArgumentOutOfRangeException();
            }
            
            IncrementIP();
        }
    }

    public void Run(Chunk chunk)
    {
        _current = chunk;
        _ip = 0;
        // Iterate through the current chunk and execute
        
        Interpret();
        
        // Top of stack is result
        //Console.WriteLine("Top of stack: " + ToString(_valueStack.Peek()));
        
        // Stored variables:
        foreach (var entry in _globals)
        {
            Console.WriteLine($"Variable \"{entry.Key.GetValue()}\": {entry.Value.GetValue()}");
        }
    }
}