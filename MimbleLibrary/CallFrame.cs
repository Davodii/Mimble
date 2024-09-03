using Mimble.Functions;

namespace Mimble;

public class CallFrame(UserDefined function, Environment environment)
{
    public UserDefined function { get; } = function;
    private int _ip;

    public byte ReadByte()
    {
        return function.GetChunk().GetByte(_ip++);
    }

    // ReSharper disable once InconsistentNaming
    public int GetIP()
    {
        return _ip;
    }

    public void AddOffset(int offset)
    {
        _ip += offset;
    }

    public Environment GetEnvironment()
    {
        return environment;
    }

    public void SetEnvironment(Environment environment1)
    {
        environment = environment1;
    }

    public override string ToString()
    {
        return $"<Frame for function [{function}]>";
    }
}