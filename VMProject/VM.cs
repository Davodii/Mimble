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

    private void Push(Value value)
    {
        _valueStack.Push(value);
    }

    private Value Pop()
    {
        return _valueStack.Pop();
    }

    private Value Peek()
    {
        return _valueStack.Peek();
    }
    
    private byte ReadByte()
    {
        return _current.GetByte(_ip++);
    }
    private short ReadShort()
    {
        // read the next two bytes (aaaa bbbb)
        //                           b1   b2
        byte b1 = ReadByte();
        byte b2 = ReadByte();
        
        return (short)((b1 << 8) | b2);
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

    private bool IsFalsey(Value val)
    {
        return val.GetValueType() == ValueType.Boolean && (bool)val.GetValue() == false;
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
            Instruction instruction = (Instruction)ReadByte();

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
                    Value constant = _current.GetLocal(ReadByte());
                    Push(constant);
                    break;
                case Instruction.Add: // +
                    Add();
                    break;
                case Instruction.Subtract: // -
                    Subtract();
                    break;
                case Instruction.Multiply: // *
                    Multiply();
                    break;
                case Instruction.Divide: // /
                    Divide();
                    break;
                case Instruction.Negate: // -
                    Negate();
                    break;
                case Instruction.Equal: // ==
                {
                    Value val1 = Pop();
                    Value val2 = Pop();
                    
                    Push(new Value(val1.Equals(val2), ValueType.Boolean));
                    
                    break;
                }
                case Instruction.Greater: // >
                {
                    Value val1 = Pop();
                    Value val2 = Pop();

                    if (!AreNumbers(val1, val2))
                    {
                        // TODO: error
                    }

                    bool greater = (double)val2.GetValue() > (double)val1.GetValue();
                    Push(new Value(greater, ValueType.Boolean));
                    break;
                }
                case Instruction.Less: // <
                {
                    Value val1 = Pop();
                    Value val2 = Pop();

                    if (!AreNumbers(val1, val2))
                    {
                        // TODO: error
                    }

                    bool greater = (double)val2.GetValue() < (double)val1.GetValue();
                    Push(new Value(greater, ValueType.Boolean));
                    break;
                }
                case Instruction.Not: // not
                {
                    Value val = Pop();

                    if (val.GetValueType() != ValueType.Boolean)
                    {
                        // TODO: Error
                    }

                    bool not = !(bool)val.GetValue();
                    Push(new Value(not, ValueType.Boolean));
                    break;
                }
                case Instruction.Jump:
                {
                    // Read the next two bytes
                    short offset = ReadShort();

                    // Add the offset to the ip
                    _ip += offset;
                    break;
                }
                case Instruction.JumpIfFalse:
                {
                    // Check if the top value is falsey
                    if (IsFalsey(Peek()))
                    {
                        // Jump
                        short offset = ReadShort();
                        _ip += offset;
                    }
                    else
                    {
                        _ip += 2;
                    }

                    break;
                }
                case Instruction.Loop:
                {
                    short offset = ReadShort();
                    
                    // Subtract offset from the ip (going back up the code
                    _ip -= offset;
                    break;
                }
                case Instruction.Call:
                    break;
                case Instruction.Return:
                    break;
                case Instruction.And:
                    break;
                case Instruction.Or:
                    break;
                case Instruction.StoreVar:
                {
                    // Pop the top of the stack and store it into the variable given by the second 
                    Value val = Pop();

                    // IP now points to the index of the variable name
                    byte index = ReadByte();
                    Value varIdentifier = _current.GetLocal(index);

                    // Store the value inside the identifier in the list
                    _globals.Add(varIdentifier, val);
                    break;
                }
                case Instruction.LoadVar:
                    break;
                case Instruction.End:
                    return;
                default:
                    throw new ArgumentOutOfRangeException();
            }
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