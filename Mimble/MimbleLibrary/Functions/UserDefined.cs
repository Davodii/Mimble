using Mimble.Values;
using ValueType = Mimble.Values.ValueType;

namespace Mimble.Functions;

public class UserDefined(string identifier, Chunk chunk) : Function(identifier)
{
    public Chunk Chunk { get;  } = chunk;

    public override void PrintCode()
    {
        Console.WriteLine(this.ToString());
        chunk.PrintChunk();
        Console.WriteLine();
        
        // Print any defined functions
        for (int i = 0; i < Chunk.GetConstantCount(); i++)
        {
            Value val = Chunk.GetConstant(i);

            if (val.GetValueType() == ValueType.UserDefinedFunction || val.GetValueType() == ValueType.NativeFunction)
            {
                ((Function)val.GetValue()).PrintCode();
            }
        }
    }
}