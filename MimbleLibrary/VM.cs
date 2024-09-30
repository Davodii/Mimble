using Mimble.Exceptions;
using Mimble.Functions;
using Mimble.Values;
using ValueType = Mimble.Values.ValueType;

namespace Mimble;

// ReSharper disable once InconsistentNaming
public class VM
{
    /* 
     * Perform interpreting for any passed in chunks of code.
     */

    private readonly Stack<Value> _valueStack = new();
    private readonly Stack<CallFrame> _frames = new();

    private UserDefined CurrentFunction()
    {
        return CurrentFrame().function;
    }

    private CallFrame CurrentFrame()
    {
        return _frames.Peek();
    }

    public void Push(Value value)
    {
        _valueStack.Push(value);
    }

    public Value Pop()
    {
        return _valueStack.Pop();
    }

    public Value Peek()
    {
        return _valueStack.Peek();
    }
    
    private byte ReadByte()
    { 
        return _frames.Peek().ReadByte();
    }
    
    private short ReadShort()
    {
        // read the next two bytes (aaaa bbbb)
        //                           a    b
        byte b1 = ReadByte();
        byte b2 = ReadByte();
        
        return (short)((b1 << 8) | b2);
    }

    public int CurrentLineNumber()
    {
        return CurrentFunction().GetChunk().GetLine(CurrentFrame().GetIP());
    }
    
    #region Utility Functions

    private bool IsFalse(ConstantValue val)
    {
        if (!ConstantValue.IsBoolean(val))
        {
            throw new RunTimeException(CurrentLineNumber(), "Expected value to be a boolean.");
        }
        return ((BooleanValue)val).AsBoolean() == false;
    }

    // ReSharper disable once UnusedMember.Local
    private void PrintStackTrace()
    {
        Console.WriteLine("Value Stack:");
        
        // Print the contents of the values stack
        foreach (var value in _valueStack)
        {
            Console.Write($" [{value}] ");
        }

        Console.WriteLine();
    }

    // ReSharper disable once UnusedMember.Local
    private void PrintCallStack()
    {
        Console.WriteLine("Frame Stack:");
        foreach (var frame in _frames)
        {
            Console.Write($" [{frame}] ");
        }

        Console.WriteLine();
    }
    
    #endregion
    
    #region Binary Operations

    private void NumberBinaryOp(Instruction op)
    {
        Value val1 = Pop();
        Value val2 = Pop();

        if (op == Instruction.Add && (ConstantValue.IsString(val1) || ConstantValue.IsString(val2)))
        {
            // Perform string concatenation
            string sResult = val2.ToString() + val1;
            
            // push the result back onto the stack
            Push(new StringValue(sResult));
            return;
        }

        double val1Double, val2Double;
        
        try
        {
            val1Double = ((NumberValue)val1).AsNumber();
            val2Double = ((NumberValue)val2).AsNumber();
        }
        catch (ConversionException e)
        {
            throw new RunTimeException(CurrentLineNumber(), $"Expected '{e.Expected}' value but got '{e.VValue.GetValueType()}'.");
        }
        
        // ! add, sub, mul, div
        switch (op)
        {
            case Instruction.Less:
                Push(new BooleanValue(val2Double < val1Double));
                break;
            case Instruction.Greater:
                Push(new BooleanValue(val2Double > val1Double));
                break;
            case Instruction.Add:
                Push(new NumberValue(val2Double + val1Double));
                break;
            case Instruction.Subtract:
                Push(new NumberValue(val2Double - val1Double));
                break;
            case Instruction.Multiply:
                Push(new NumberValue(val2Double * val1Double));
                break;
            case Instruction.Divide:
                Push(new NumberValue(val2Double / val1Double));
                break;
            default:
                // Should be unreachable
                throw new RunTimeException(CurrentLineNumber(), $"Not a binary operator: '{op}'");
        }
    }
    
    private void Or()
    {
        Value val1 = Pop();
        Value val2 = Pop();
                    
        if (!ConstantValue.IsBoolean(val1) || !ConstantValue.IsBoolean(val2))
        {
            throw new RunTimeException(CurrentLineNumber(), "Expected two booleans.");
        }

        if (IsFalse((ConstantValue)val1) && IsFalse((ConstantValue)val2))
        {
            Push(new BooleanValue(false));
        }
        else
        {
            Push(new BooleanValue(true));
        }
    }

    private void And()
    {
        Value val1 = Pop();
        Value val2 = Pop();

        if (!ConstantValue.IsBoolean(val1) || !ConstantValue.IsBoolean(val2))
        {
            throw new RunTimeException(CurrentLineNumber(), "Expected two booleans.");
        }

        if (IsFalse((ConstantValue)val2) || IsFalse((ConstantValue)val1))
        {
            // TODO: Skip over val1
            Push(new BooleanValue(false));
        }
        else
        {
            Push(new BooleanValue(true));
        }
    }

    private void In()
    {
        ListValue list = (ListValue)Pop();
        Value val = Pop();

        Push(new BooleanValue(list.Contains(val)));
    }
    
    private void Equal()
    {
        Value val1 = Pop();
        Value val2 = Pop();
                    
        Push(new BooleanValue(val1.Equals(val2)));
    }
    
    #endregion
    
    #region Unary Operations

    private void Negate()
    {
        Value val = Pop();
        if (!ConstantValue.IsNumber(val))
        {
            throw new RunTimeException(CurrentLineNumber(), $"Expected a number but got '{val.GetValueType()}'.");
        }

        double result = -((NumberValue)val).AsNumber();
        Push(new NumberValue(result));
    }

    private void Not()
    {
        Value val = Pop();

        if (!ConstantValue.IsBoolean(val))
        {
            throw new RunTimeException(CurrentLineNumber(), $"Expected a boolean but got '{val.GetValueType()}'");
        }

        bool not = !((BooleanValue)val).AsBoolean();
        Push(new BooleanValue(not));

    }
    #endregion
    
    #region Functions
    
    private void DefineFunction()
    {
        FunctionValue functionValue = (FunctionValue)CurrentFunction().GetChunk().GetConstant(ReadByte());
        string identifier = functionValue.GetValue().identifier;

        // Check to see if the function is already defined
        if (CurrentFrame().GetEnvironment().Defined(identifier))
        {
            throw new RunTimeException(CurrentLineNumber(), "Identifier is already defined.");
        }

        CurrentFrame().GetEnvironment().Assign(identifier, functionValue);
    }
    
    private void CallFunction()
    {
        // Get the corresponding FunctionValue
        FunctionValue functionValue = (FunctionValue)Pop();
        Function function = functionValue.GetValue();
                    
        // Get the arity
        int argumentCount = ReadByte();

        if (argumentCount != function.arity)
        {
            string relative;
            if (argumentCount < function.arity) relative = "too few";
            else if (argumentCount > function.arity) relative = "too many";
            else relative = "no";
            throw new RunTimeException(CurrentLineNumber(), $"The function was called with {relative} arguments.");
        }

        if (functionValue.GetValueType() == ValueType.UserDefinedFunction)
        {
            Environment environment = new Environment(CurrentFrame().GetEnvironment());
            CallFrame frame = new CallFrame((UserDefined)function, environment);
                        
            // Add the frame onto the call stack
            _frames.Push(frame);
            return;
        }
        if (functionValue.GetValueType() == ValueType.NativeFunction)
        {
            // Call the native function
            ((Native)function).Execute(this);
            return;
        }
                    
        throw new RunTimeException(CurrentLineNumber(), $"Identifier '{functionValue.GetValueType()}' is not a function.");
    }
    
    #endregion
    
    private void Execute()
    {
        for (;;)
        {
            // Given a current chunk, interpret the code
            // Read the current instruction
            Instruction instruction = (Instruction)ReadByte();

            // PrintStackTrace();
            // PrintCallStack();
            switch (instruction)
            {
                case Instruction.Pop:
                    Pop();
                    break;
                case Instruction.Null:
                    Push(NullValue.GetNullValue());
                    break;
                case Instruction.False:
                    Push(new BooleanValue(false));
                    break;
                case Instruction.True:
                    Push(new BooleanValue(true));
                    break;
                case Instruction.LoadConstant:
                    Value constant = CurrentFunction().GetChunk().GetConstant(ReadByte());
                    Push(constant);
                    break;
                case Instruction.Negate: // -
                    Negate();
                    break;
                case Instruction.Not: // not
                    Not();
                    break;
                case Instruction.Add:
                case Instruction.Subtract:
                case Instruction.Multiply:
                case Instruction.Divide:
                case Instruction.Greater:
                case Instruction.Less:
                    NumberBinaryOp(instruction);
                    break;
                case Instruction.Equal: // ==
                    Equal();
                    break;
                case Instruction.And:
                    And();
                    break;
                case Instruction.Or:
                    Or();
                    break;
                case Instruction.In:
                    In();
                    break;
                case Instruction.Jump:
                {
                    // Read the next two bytes
                    short offset = ReadShort();

                    // Add the offset to the ip
                    CurrentFrame().AddOffset(offset);
                    break;
                }
                case Instruction.JumpIfFalse:
                {
                    // Check if the top value is false
                    if (IsFalse((ConstantValue)Peek()))
                    {
                        // Jump
                        short offset = ReadShort();
                        CurrentFrame().AddOffset(offset);
                    }
                    else
                    {
                        // Skip the bytes of the jump instruction
                        CurrentFrame().AddOffset(2);
                    }

                    break;
                }
                case Instruction.Loop:
                {
                    short offset = ReadShort();
                    
                    // Subtract offset from the ip (going back up the code
                    CurrentFrame().AddOffset(-offset);
                    break;
                }
                case Instruction.BeginScope:
                {
                    Environment scope = new Environment(CurrentFrame().GetEnvironment());
                    CurrentFrame().SetEnvironment(scope);
                    break;
                }
                case Instruction.EndScope:
                {
                    Environment? enclosing = CurrentFrame().GetEnvironment().GetEnclosing();
                    
                    if (enclosing == null)
                    {
                        // ! Error - cannot go out of this scope since there is no scope above it
                        throw new RunTimeException(CurrentLineNumber(), "Cannot end scope, no enclosing scope.");
                    }

                    CurrentFrame().SetEnvironment(enclosing);
                    
                    break;
                }
                case Instruction.StoreVar:
                {
                    // Pop the top of the stack and store it into the variable given by the second 
                    Value val = Pop(); // to store inside the variable

                    StringValue identifier = (StringValue)CurrentFunction().GetChunk().GetConstant(ReadByte());
                    
                    // Store the value inside the identifier in the list
                    CurrentFrame().GetEnvironment().Assign(identifier.AsString(), val);
                    
                    Push(val);
                    break;
                }
                case Instruction.LoadVar:
                {
                    StringValue identifier = (StringValue)CurrentFunction().GetChunk().GetConstant(ReadByte());
                    
                    Value value = CurrentFrame().GetEnvironment().Get(identifier.AsString());
                    
                    Push(value);
                    break;
                }
                case Instruction.DefFunction:
                    DefineFunction();
                    break;
                case Instruction.Call:
                    CallFunction();
                    break;
                case Instruction.Return:
                {
                    // Return the current function
                    Value value = Pop();
                    
                    _frames.Pop();
                    if (_frames.Count == 0)
                    {
                        // Finished execution
                        return;
                    }
                    
                    Push(value);
                    break;
                }
                case Instruction.CreateListFromValues:
                {
                    byte count = ReadByte();

                    ListValue list = new ListValue(_valueStack, count);
                    
                    Push(list);
                    break;
                }
                case Instruction.CreateListFromRange:
                {
                    Value increment = Pop();
                    Value end = Pop();
                    Value start = Pop();

                    if (!ConstantValue.IsNumber(increment) || !ConstantValue.IsNumber(end) || !ConstantValue.IsNumber(start))
                    {
                        // TODO: make this return something actually useful
                        throw new RunTimeException(CurrentLineNumber(), "Expected numbers...");
                    }
                    
                    // TODO: check the correct order of start, end and increment are correct
                    // i.e. if end > start, increment > 0
                    // if end < start, increment > 1
                    // TODO: check that the increment will actually has the correct sign

                    ListValue list = new ListValue(((NumberValue)start).AsInteger(), ((NumberValue)end).AsInteger(),
                        ((NumberValue)increment).AsInteger());
                    
                    Push(list);
                    break;
                }
                case Instruction.GetSubscript:
                {
                    NumberValue index = (NumberValue)Pop();
                    ListValue list = (ListValue)Pop();

                    Value atIndex = list.Get(index.AsInteger());
                    
                    Push(atIndex);
                    break;
                }
                case Instruction.StoreSubscript:
                {
                    Value toStore = Pop();
                    NumberValue index = (NumberValue)Pop();
                    ListValue list = (ListValue)Pop();
                    
                    list.AssignAt(index.AsInteger(), toStore);
                    Push(toStore);
                    break;
                }
                case Instruction.CreateIterator:
                {
                    ListValue list = (ListValue)Pop();
                    IteratorValue iterator = new IteratorValue(list);
                    Push(iterator);
                    break;
                }
                case Instruction.ForwardIterator:
                {
                    IteratorValue iterator = (IteratorValue)Peek();
                    try
                    {
                        Value next = iterator.GetNext();
                        Push(next);
                        CurrentFrame().AddOffset(2); // skip over the jump bytes
                    }
                    catch (IndexOutOfRangeException)
                    {
                        // iterator has finished, jump to the next part of the code
                        short jump = ReadShort();
                        CurrentFrame().AddOffset(jump);
                    }
                    break;
                }
                default:
                    throw new ArgumentOutOfRangeException();
            }
        }
    }

    public void Run(UserDefined mainFunction, Environment global)
    {
        // Initialize the main function frame
        Environment mainEnvironment = new Environment(global);
        _frames.Push(new CallFrame(mainFunction, mainEnvironment));

        Execute();
    }
}