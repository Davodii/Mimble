using System.Globalization;
using VMProject.Exceptions;
using VMProject.Functions;

namespace VMProject;

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
        return CurrentFrame().Function;
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
        return CurrentFunction().Chunk.GetLine(CurrentFrame().GetIP());
    }
    
    #region Utility Functions
    
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
                return val.AsNumber().ToString(CultureInfo.InvariantCulture);
            case ValueType.String:
                return val.AsString();
        }
        
        // Should never reach here
        throw new RunTimeException(CurrentLineNumber(), $"Expected a value but got '{val.GetValueType().ToString()}'.");
    }

    private bool IsFalse(Value val)
    {
        return val.AsBoolean() == false;
    }

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

        if (op == Instruction.Add && (Value.IsString(val1) || Value.IsString(val2)))
        {
            // Perform string concatenation
            string sResult = ToString(val2) + ToString(val1);
            
            // push the result back onto the stack
            Push(new Value(sResult, ValueType.String));
            return;
        }

        double val1Double, val2Double;
        
        try
        {
            val1Double = val1.AsNumber();
            val2Double = val2.AsNumber();
        }
        catch (ConversionException e)
        {
            throw new RunTimeException(CurrentLineNumber(), $"Expected '{e.Expected}' value but got '{e.VValue.GetValueType()}'.");
        }
        
        // ! add, sub, mul, div
        switch (op)
        {
            case Instruction.Less:
                Push(new Value(val2Double < val1Double, ValueType.Boolean));
                break;
            case Instruction.Greater:
                Push(new Value(val2Double > val1Double, ValueType.Boolean));
                break;
            case Instruction.Add:
                Push(new Value(val2Double + val1Double, ValueType.Number));
                break;
            case Instruction.Subtract:
                Push(new Value(val2Double - val1Double, ValueType.Number));
                break;
            case Instruction.Multiply:
                Push(new Value(val2Double * val1Double, ValueType.Number));
                break;
            case Instruction.Divide:
                Push(new Value(val2Double / val1Double, ValueType.Number));
                break;
        }
        
        
        // Should be unreachable
        throw new RunTimeException(CurrentLineNumber(), "Not a binary operator.");
    }
    
    private void Or()
    {
        Value val1 = Pop();
        Value val2 = Pop();
                    
        if (!Value.IsBoolean(val1) || !Value.IsBoolean(val2))
        {
            throw new RunTimeException(CurrentLineNumber(), "Expected two booleans.");
        }

        if (IsFalse(val1) && IsFalse(val2))
        {
            Push(new Value(false, ValueType.Boolean));
        }
        else
        {
            Push(new Value(true, ValueType.Boolean));
        }
    }

    private void And()
    {
        Value val1 = Pop();
        Value val2 = Pop();

        if (!Value.IsBoolean(val1) || !Value.IsBoolean(val2))
        {
            throw new RunTimeException(CurrentLineNumber(), "Expected two booleans.");
        }

        if (IsFalse(val2) || IsFalse(val1))
        {
            // Skip over val1
            Push(new Value(false, ValueType.Boolean));
        }
        else
        {
            Push(new Value(true, ValueType.Boolean));
        }
    }
    
    private void Equal()
    {
        Value val1 = Pop();
        Value val2 = Pop();
                    
        Push(new Value(val1.Equals(val2), ValueType.Boolean));
    }
    
    #endregion
    
    #region Unary Operations

    private void Negate()
    {
        Value val = Pop();
        if (val.GetValueType() != ValueType.Number)
        {
            throw new RunTimeException(CurrentLineNumber(), $"Expected a number but got '{val.GetValueType()}'.");
        }

        double result = -val.AsNumber();
        Push(new Value(result, ValueType.Number));
    }

    private void Not()
    {
        Value val = Pop();

        if (val.GetValueType() != ValueType.Boolean)
        {
            throw new RunTimeException(CurrentLineNumber(), $"Expected a boolean but got '{val.GetValueType()}'");
        }

        bool not = !(bool)val.GetValue();
        Push(new Value(not, ValueType.Boolean));

    }
    #endregion
    
    #region Functions
    
    private void DefineFunction()
    {
        Value functionValue = CurrentFunction().Chunk.GetConstant(ReadByte());
        string identifier = ((Function)functionValue.GetValue()).Identifier;

        // Check to see if the function is already defined
        if (CurrentFrame().GetEnvironment().Defined(identifier))
        {
            throw new RunTimeException(CurrentLineNumber(), "Identifier is already defined.");
        }

        CurrentFrame().GetEnvironment().Assign(identifier, functionValue);
    }
    
    private void CallFunction() 
    {
        // Function value already stored on the stack
        Value functionValue = Pop();
        Function function = (Function)functionValue.GetValue();
                    
        // Get the arity
        int argumentCount = ReadByte();

        if (argumentCount != function.Arity)
        {
            string relative;
            if (argumentCount < function.Arity) relative = "too few";
            else if (argumentCount > function.Arity) relative = "too many";
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
    
    private void Run()
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
                    Push(new Value(null!, ValueType.Null));
                    break;
                case Instruction.False:
                    Push(new Value(false, ValueType.Boolean));
                    break;
                case Instruction.True:
                    Push(new Value(true, ValueType.Boolean));
                    break;
                case Instruction.LoadConstant:
                    Value constant = CurrentFunction().Chunk.GetConstant(ReadByte());
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
                    if (IsFalse(Peek()))
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
                    Value val = Pop();

                    // IP now points to the index of the variable name
                    byte index = ReadByte();
                    Value variable = CurrentFunction().Chunk.GetConstant(index);
                    string identifier = (string)variable.GetValue();

                    // Store the value inside the identifier in the list
                    CurrentFrame().GetEnvironment().Assign(identifier, val);
                    
                    Push(val);
                    break;
                }
                case Instruction.LoadVar:
                {
                    Value variable = CurrentFunction().Chunk.GetConstant(ReadByte());
                    string identifier = (string)variable.GetValue();
                    
                    Value value = CurrentFrame().GetEnvironment().Get(identifier);
                    
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
                    
                    // ! Might be memory leak on the stack since the function may just be called 
                    // ! like 'functionCall()' but if it does not return a value, then 'null' will be
                    // ! pushed to the stack
                    Push(value);
                    break;
                }
                case Instruction.CreateListFromValues:
                {
                    byte count = ReadByte();

                    ListValue list = new ListValue(_valueStack, count);

                    Value value = new Value(list, ValueType.List);
                    Push(value);
                    break;
                }
                case Instruction.CreateListFromRange:
                {
                    Value increment = Pop();
                    Value end = Pop();
                    Value start = Pop();

                    if (!Value.IsNumber(increment) || !Value.IsNumber(end) || !Value.IsNumber(start))
                    {
                        // TODO: make this return something actually useful
                        throw new RunTimeException(CurrentLineNumber(), "Expected numbers...");
                    }
                    
                    // TODO: check the correct order of start, end and increment are correct
                    // i.e. if end > start, increment > 0
                    // if end < start, increment > 1
                    // TODO: check that the increment will actually has the correct sign

                    ListValue list = new ListValue(start.AsInteger(), end.AsInteger(),
                        increment.AsInteger());
                    
                    Value value = new Value(list, ValueType.List);
                    Push(value);
                    break;
                }
                default:
                    throw new ArgumentOutOfRangeException();
            }
        }
    }

    public void Interpret(string source)
    {
        // Compile the source program
        Compiler compiler = new Compiler();
        UserDefined mainFunction = compiler.Compile(source);
        
        // Create the global environment
        Environment global = GlobalScope.GetGlobalScope();
        
        // Initialize the main function frame
        Environment mainEnvironment = new Environment(global);
        _frames.Push(new CallFrame(mainFunction, mainEnvironment));
        
        // ! Testing purposes only
        // mainFunction.PrintCode();
        
        // Begin execution of the code
        Run();
        
    }
}