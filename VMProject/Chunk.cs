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
    private Value[] _constants;
    private int _constantsCapacity;
    private int _constantsCount;

    //TODO: Add some way to store the current line number
    // Use a delta-offset table
    // List of offset (from start of bytecode) and the delta change in the line number, can store it as a list of bytes

    public Chunk()
    {
        // Initialize the chunk
        _codeCount = 0;
        _codeCapacity = InitSize;
        _code = new byte[_codeCapacity];

        _constantsCount = 0;
        _constantsCapacity = 256;
        _constants = new Value[_constantsCapacity];

    }

    public void Write(byte data)
    {
        // Check if there is still room in the chunk
        if (_codeCount == _codeCapacity)
        {
            // Grow the array capacity by the GrowRate
            Array.Resize(ref _code, _codeCapacity * GrowRate);
            _codeCapacity *= GrowRate;
        }
        
        // Write the operator
        _code[_codeCount] = data;
        _codeCount++;
    }

    public int AddConstant(Value value)
    {
        if (_constantsCount >= _constantsCapacity)
        {
            //TODO: throw error
            return -1;
        }

        _constants[_constantsCount] = value;
        //TODO: check this works... 
        return _constantsCount++;
    }

    public Value GetConstant(int index)
    {
        if (index >= _constantsCount)
        {
            // ! out of range exception
            return null;
        }

        return _constants[index];
    }

    public int GetCodeCount()
    {
        return _codeCount;
        
    }
    
    public byte GetByte(int index)
    {
        if (index >= _codeCount)
        {
            // ! out of range error
            return 255;
        }

        return _code[index];
    }
    
    #region Utility Functions

    public void PrintChunk()
    {
        for (int i = 0; i < _codeCount; i++)
        {
            switch ((Instruction)_code[i])
            {
                case Instruction.Pop:
                    Console.WriteLine("Pop");
                    break;
                case Instruction.Null:
                    Console.WriteLine("Null");
                    break;
                case Instruction.False:
                    Console.WriteLine("[ false ]");
                    break;
                case Instruction.True:
                    Console.WriteLine("[ true ]");
                    break;
                case Instruction.Constant:
                    i++;
                    Console.WriteLine($"[ {_code[i]}: {GetConstant(_code[i]).GetValue()}]");
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
                case Instruction.End:
                    Console.WriteLine("End");
                    return;
                default:
                    throw new ArgumentOutOfRangeException();
            }
        }
    }
    
    #endregion
}