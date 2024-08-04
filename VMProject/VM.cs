using System.Collections;
using System.Runtime.Intrinsics.X86;

namespace VMProject;

public class VM
{
    /* 
     * Perform interpreting for any passed in chunks of code.
     */

    private Stack<Value> _valueStack;
    private Stack<CallFrame> _frames; // the bottom-most frame will be the "global" function
    
    
    public VM()
    {
        _valueStack = new Stack<Value>();
        _frames = new Stack<CallFrame>();
        
        // TODO: get the first callFrame
    }

    private Function CurrentFunction()
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
        
        // Define the function inside the current environemnt
        _frames.Peek().GetEnvironment().Assign(function.Identifier, value);
    }
    
    #endregion
    
    private void Run()
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
                case Instruction.And:
                {
                    Value val1 = Pop();
                    Value val2 = Pop();

                    if (!AreBoolean(val1, val2))
                    {
                        // ! Error
                    }

                    if (IsFalsey(val2) || IsFalsey(val1))
                    {
                        // Skip over val1
                        Push(new Value(false, ValueType.Boolean));
                    }
                    else
                    {
                        Push(new Value(true, ValueType.Boolean));
                    }
                    
                    break;
                }
                case Instruction.Or:
                {
                    Value val1 = Pop();
                    Value val2 = Pop();
                    
                    if (!AreBoolean(val1, val2))
                    {
                        // ! Error
                    }

                    if (IsFalsey(val1) || IsFalsey(val2))
                    {
                        Push(new Value(false, ValueType.Boolean));
                    }
                    else
                    {
                        Push(new Value(true, ValueType.Boolean));
                    }
                    
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
                    CurrentFrame().AddOffset(offset);
                    break;
                }
                case Instruction.JumpIfFalse:
                {
                    // Check if the top value is false
                    if (IsFalsey(Peek()))
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
                {
                    Value functionValue = CurrentFunction().Chunk.GetConstant(ReadByte());
                    string identifier = ((Function)functionValue.GetValue()).Identifier;

                    // Check to see if the function is already defined
                    if (CurrentFrame().GetEnvironment().Defined(identifier))
                    {
                        // ! Error
                        return;
                    }

                    CurrentFrame().GetEnvironment().Assign(identifier, functionValue);
                    break;
                }
                case Instruction.Call:
                {
                    // Function value already stored on the stack
                    Value functionValue = Pop();

                    if (functionValue.GetValueType() != ValueType.Function)
                    {
                        // ! Error
                    }

                    // TODO: make this work with the main function 
                    Environment environment = new Environment(CurrentFrame().GetEnvironment());

                    CallFrame frame = new CallFrame((Function)functionValue.GetValue(), environment);
                    
                    // Add the frame onto the call stack
                    _frames.Push(frame);
                    
                    break;
                }
                case Instruction.Return:
                {
                    // Return the current function
                    Value value = Pop();

                    if (_frames.Count == 0)
                    {
                        // Finished execution
                        return;
                    }

                    _frames.Pop();
                    
                    // TODO: might be memory leak on the stack since the function may just be called 
                    // like 'functionCall()' but if it does not return a value, then 'null' will be
                    // pushed to the stack
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
        // Iterate through the current chunk and execute

        Compiler compiler = new Compiler();

        Function mainFunction = compiler.Compile(source);
        Environment mainEnvironment = new Environment();
        
        _frames.Push(new CallFrame(mainFunction, mainEnvironment));
        
        Run();

        // The main function code should be run by now ... (hopefully)
    }
}