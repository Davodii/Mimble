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
    private List<Value> _locals;

    //TODO: Add some way to store the current line number
    // Use a delta-offset table
    // List of offset (from start of bytecode) and the delta change in the line number, can store it as a list of bytes

    public Chunk()
    {
        // Initialize the chunk
        _codeCount = 0;
        _codeCapacity = InitSize;
        _code = new byte[_codeCapacity];
        _locals = new List<Value>();

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

    public int AddLocal(Value value)
    {

        _locals.Add(value);
        //TODO: check this works... 
        return _locals.Count - 1;
    }

    public Value GetLocal(int index)
    {
        if (index >= _locals.Count)
        {
            // ! out of range exception
            return null;
        }

        return _locals[index];
    }
    
    
    public int GetLocalIndex(Value constant)
    {
        //TODO: check that this will work if constant is not found
        return _locals.IndexOf(constant);
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
                case Instruction.LoadConstant:
                    i++;
                    Console.WriteLine($"[ {_code[i]}: {GetLocal(_code[i]).GetValue()}]");
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