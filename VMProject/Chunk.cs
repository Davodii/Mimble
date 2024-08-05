namespace VMProject;

public class Chunk
{
    private const int InitSize = 8;
    private const int GrowRate = 2;

    // Store the array of bytecode instructions
    private byte[] _code;
    private int _codeCapacity;
    private int _codeCount;
    
    // Store constant values found within this chunk
    // Also used to get variable identifiers
    private readonly List<Value> _constants;

    // Store info about the line regarding each index
    private readonly LineNumberTable<int> _lineNumberTable = new LineNumberTable<int>();
    
    public Chunk()
    {
        // Initialize the chunk
        _codeCount = 0;
        _codeCapacity = InitSize;
        _code = new byte[_codeCapacity];
        _constants = new List<Value>();

    }

    public void Write(byte data, int line)
    {
        // Check if there is still room in the chunk
        if (_codeCount == _codeCapacity)
        {
            // Grow the array capacity by the GrowRate
            Array.Resize(ref _code, _codeCapacity * GrowRate);
            _codeCapacity *= GrowRate;
        }
        
        // Add the line to the info
        _lineNumberTable.AddIndexToLine(line, _codeCount);
        
        // Write the operator
        _code[_codeCount] = data;
        _codeCount++;
    }

    public void Write(int offset, byte data)
    {
        if (offset >= _codeCount)
        {
            throw new IndexOutOfRangeException( "Offset outside the initialized array of code.");
        }
        
        _code[offset] = data;
    }

    public int AddConstant(Value value)
    {
        _constants.Add(value);
        return _constants.Count - 1;
    }

    public Value GetConstant(int index)
    {
        if (index >= _constants.Count)
        {
            throw new IndexOutOfRangeException("Index outside of code array range.");
        }

        return _constants[index];
    }

    private bool ContainsValue(Token token)
    {
        return _constants.Any(value => value.GetValueType() ==  ValueType.Identifier && (string)value.GetValue() == token.GetValue());
    }

    public int GetConstantIndex(Token token)
    {
        if (!ContainsValue(token)) throw new CompileTimeException(token, "Current chunk does not contain the token.");
        
        // Get the index of the first Value to have the same value as the token and is an identifier
        return _constants.FindIndex(value =>
            value.GetValueType() == ValueType.Identifier && (string)value.GetValue() == token.GetValue());
    }

    public int GetCodeCount()
    {
        return _codeCount;
    }
    
    public byte GetByte(int index)
    {
        if (index >= _codeCount)
        {
            throw new IndexOutOfRangeException("Index out of range of code array.");
        }

        return _code[index];
    }
    
    #region Utility Functions

    public void PrintChunk()
    {
        for (int i = 0; i < _codeCount; i++)
        {
            Console.Write($"{_lineNumberTable.GetLineInfo(i).ToString(),3}-{i.ToString(),-4} : ");
            switch ((Instruction)_code[i])
            {
                case Instruction.Pop:
                    Console.WriteLine("Pop");
                    break;
                case Instruction.Null:
                    Console.WriteLine("[ Null ]");
                    break;
                case Instruction.False:
                    Console.WriteLine("[ false ]");
                    break;
                case Instruction.True:
                    Console.WriteLine("[ true ]");
                    break;
                case Instruction.LoadConstant:
                    i++;
                    Console.WriteLine("Load Constant {" + _code[i] + "}");
                    break;
                case Instruction.Add:
                    Console.WriteLine("Add");
                    break;
                case Instruction.Subtract:
                    Console.WriteLine("Sub");
                    break;
                case Instruction.Multiply:
                    Console.WriteLine("Mul");
                    break;
                case Instruction.Divide:
                    Console.WriteLine("Div");
                    break;
                case Instruction.Negate:
                    Console.WriteLine("Neg");
                    break;
                case Instruction.Equal:
                    Console.WriteLine("Equal");
                    break;
                case Instruction.Greater:
                    Console.WriteLine("Greater");
                    break;
                case Instruction.Less:
                    Console.WriteLine("Less");
                    break;
                case Instruction.Not:
                    Console.WriteLine("Not");
                    break;
                case Instruction.Jump:
                    Console.WriteLine($"Jump +0x{_code[++i].ToString("X2")}{_code[i].ToString("X2")}");
                    break;
                case Instruction.JumpIfFalse:
                    Console.WriteLine($"Jump if false +0x{_code[++i].ToString("X2")}{_code[++i].ToString("X2")}");
                    i += 2;
                    break;
                case Instruction.Loop:
                    Console.WriteLine($"Loop -{_code[++i].ToString("X2")}{_code[i].ToString("X2")}");
                    break;
                case Instruction.Call:
                    Console.WriteLine($"Call {_code[++i]}");
                    break;
                case Instruction.Return:
                    Console.WriteLine("Return");
                    break;
                case Instruction.And:
                    Console.WriteLine("And");
                    break;
                case Instruction.Or:
                    Console.WriteLine("Or");
                    break;
                case Instruction.StoreVar:
                    Console.WriteLine($"Store Var [{_code[++i].ToString("X")}]");
                    break;
                case Instruction.LoadVar:
                    Console.WriteLine($"Load Var [{_code[++i].ToString("X2")}]");
                    break;
                case Instruction.DefFunction:
                    Console.WriteLine($"Define Function [{_code[++i].ToString("X2")}]");
                    break;
                case Instruction.BeginScope:
                    Console.WriteLine("Begin Scope");
                    break;
                case Instruction.EndScope:
                    Console.WriteLine("End Scope");
                    break;
                default:
                    Console.WriteLine("Unexpected instruction / value: " + i);
                    break;
            }
        }
    }
    
    #endregion
}