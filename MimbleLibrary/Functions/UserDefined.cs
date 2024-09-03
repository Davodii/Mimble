using Mimble.Values;
using ValueType = Mimble.Values.ValueType;

namespace Mimble.Functions;

public class UserDefined(string identifier, Chunk chunk) : Function(identifier)
{

    public override void PrintCode()
    {
        Console.WriteLine(this.ToString());
        chunk.PrintChunk();
        Console.WriteLine();
        
        // Print any defined functions
        for (int i = 0; i < chunk.GetConstantCount(); i++)
        {
            Value val = chunk.GetConstant(i);

            if (val.GetValueType() == ValueType.UserDefinedFunction || val.GetValueType() == ValueType.NativeFunction)
            {
                ((Function)val.GetValue()).PrintCode();
            }
        }
    }

    public Chunk GetChunk()
    {
        return chunk;
    }
}