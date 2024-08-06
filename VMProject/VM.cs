using System.Collections;
using System.Globalization;
using System.Runtime.Intrinsics.X86;
using VMProject.Functions;

namespace VMProject;

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
        return _frames.Peek().ReadByte();
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

    private bool AreNumbers(Value val1, Value val2)
    {
        return val1.GetValueType() == ValueType.Number && val2.GetValueType() == ValueType.Number;
    }

    private bool AreBoolean(Value val1, Value val2)
    {
        return val1.GetValueType() == ValueType.Boolean && val2.GetValueType() == ValueType.Boolean;
    }
    
    private double AsNumber(Value val)
    {
        if (val.GetValueType() != ValueType.Number)
        {
            throw new RunTimeException(0, $"Expected a number, but got '{val.GetValueType().ToString()}'.");
        }

        return (double)val.GetValue();
    }

    private string AsString(Value val)
    {
        if (val.GetValueType() != ValueType.String)
        {
            throw new RunTimeException(0, $"Expected a string, but got '{val.GetValueType().ToString()}'.");
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
                return AsNumber(val).ToString(CultureInfo.InvariantCulture);
            case ValueType.String:
                return AsString(val);
        }
        
        // Should never reach here
        throw new RunTimeException(0, $"Expected a value but got '{val.GetValueType().ToString()}'.");
    }

    private bool IsFalse(Value val)
    {
        return val.GetValueType() == ValueType.Boolean && (bool)val.GetValue() == false;
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
            Console.Write($" [{frame.ToString()}] ");
        }

        Console.WriteLine();
    }
    
    #endregion
    
    #region Binary Operations
    
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
            // ! Invalid state
            throw new RunTimeException(0, $"The values '{val2.GetValue()}' and '{val1.GetValue()}' cannot be added.");
        }
    }
    
    private void Subtract()
    {
        Value val1 = Pop();
        Value val2 = Pop();

        if (!AreNumbers(val1, val2))
        {
            throw new RunTimeException(0, "Expected two numbers.");
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
            throw new RunTimeException(0, "Expected two numbers.");
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
            throw new RunTimeException(0, "Expected two numbers.");
        }

        double result = AsNumber(val2) / AsNumber(val1);
        Push(new Value(result, ValueType.Number));
    }

    private void Or()
    {
        Value val1 = Pop();
        Value val2 = Pop();
                    
        if (!AreBoolean(val1, val2))
        {
            throw new RunTimeException(0, "Expected two booleans.");
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

        if (!AreBoolean(val1, val2))
        {
            throw new RunTimeException(0, "Expected two booleans.");
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

    private void Less()
    {
        Value val1 = Pop();
        Value val2 = Pop();

        if (!AreNumbers(val1, val2))
        {
            throw new RunTimeException(0, "Expected two numbers.");
        }

        bool greater = (double)val2.GetValue() < (double)val1.GetValue();
        Push(new Value(greater, ValueType.Boolean)); 
    }

    private void Greater()
    {
        Value val1 = Pop();
        Value val2 = Pop();

        if (!AreNumbers(val1, val2))
        {
            throw new RunTimeException(0, "Expected two numbers.");
        }

        bool greater = (double)val2.GetValue() > (double)val1.GetValue();
        Push(new Value(greater, ValueType.Boolean));
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
            throw new RunTimeException(0, $"Expected a number but got '{val.GetValueType()}'.");
        }

        double result = -AsNumber(val);
        Push(new Value(result, ValueType.Number));
    }

    private void Not()
    {
        Value val = Pop();

        if (val.GetValueType() != ValueType.Boolean)
        {
            throw new RunTimeException(0, $"Expected a boolean but got '{val.GetValueType()}'");
        }

        bool not = !(bool)val.GetValue();
        Push(new Value(not, ValueType.Boolean));

    }
    #endregion
    
    #region Functions

    private void FunctionDefinition()
    {
        // Current instruction is DefFunction
        byte functionIndex = ReadByte();
        Value value = CurrentFunction().Chunk.GetConstant(functionIndex);
        Function function = (Function)(value.GetValue());
        
        /*// Check if the current function is defined anywhere else in the environment
        if (_frames.Peek().Environment.Defined(function.Identifier))
        {
            // The function is already defined
            // ! Error
            Console.WriteLine("--- Function already defined");
            return;
        }

        Environment environment = new Environment();
        
        // Define the parameters inside the current chunk's environment
        for (int i = 0; i < function.Arity; i++)
        {
            // Get the constant's value
            Value parameter = function.Chunk.GetConstant(i);

            if (_frames.Peek().Environment.Defined((string)parameter.GetValue()))
            {
                // ! parameter already defined
                Console.WriteLine("--- Parameter already defined");
                return;
            }
            
            environment.Assign((string)parameter.GetValue(), new Value(null, ValueType.Null));
        }*/
        
        // Define the function inside the current environment
        if (_frames.Peek().GetEnvironment().Defined(function.Identifier))
        {
            // ! Identifier already defined
            Console.WriteLine($"Identifier '{function.Identifier}' already defined.");
            return;
        }
        
        // Define the function inside the current environment
        _frames.Peek().GetEnvironment().Assign(function.Identifier, value);
    }

    private void DefineFunction()
    {
        Value functionValue = CurrentFunction().Chunk.GetConstant(ReadByte());
        string identifier = ((Function)functionValue.GetValue()).Identifier;

        // Check to see if the function is already defined
        if (CurrentFrame().GetEnvironment().Defined(identifier))
        {
            throw new RunTimeException(0, "Identifier is already defined.");
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
            string relative = "not";
            if (argumentCount < function.Arity) relative = "too few";
            else if (argumentCount > function.Arity) relative = "too many";
            else relative = "no";
            throw new RunTimeException(0, $"The function was called with {relative} arguments.");
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
            ((Native)function).Execute(_valueStack);
            return;
        }
                    
        throw new RunTimeException(0, $"Identifier '{functionValue.GetValueType()}' is not a function.");
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
                    Equal();
                    break;
                case Instruction.Greater: // >
                    Greater();
                    break;
                case Instruction.Less: // <
                    Less();
                    break;
                case Instruction.And:
                    And();
                    break;
                case Instruction.Or:
                    Or();
                    break;
                case Instruction.Not: // not
                    Not();
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
                        throw new RunTimeException(0, "Cannot end scope, no enclosing scope.");
                        break;
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
        mainFunction.PrintCode();
        
        // Begin execution of the code
        Run();
        
    }
}