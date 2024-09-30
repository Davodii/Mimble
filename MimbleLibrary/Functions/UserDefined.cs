using Mimble.Values;
using ValueType = Mimble.Values.ValueType;

namespace Mimble.Functions;

public class UserDefined : Function
{
    private readonly Chunk _chunk;

    public UserDefined(string identifier, Chunk chunk) : base(identifier)
    {
        _chunk = chunk;
    }

    public override void PrintCode()
    {
        Console.WriteLine(this.ToString());
        _chunk.PrintChunk();
        Console.WriteLine();
        
        // Print any defined functions
        for (int i = 0; i < _chunk.GetConstantCount(); i++)
        {
            Value val = _chunk.GetConstant(i);

            if (val.GetValueType() == ValueType.UserDefinedFunction || val.GetValueType() == ValueType.NativeFunction)
            {
                ((Function)val.GetValue()).PrintCode();
            }
        }
    }

    public Chunk GetChunk()
    {
        return _chunk;
    }
}