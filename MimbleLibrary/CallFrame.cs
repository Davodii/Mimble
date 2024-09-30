using Mimble.Functions;

namespace Mimble;

public class CallFrame
{
    public UserDefined function { get; }
    private int _ip;
    private Environment _environment;

    public CallFrame(UserDefined function, Environment environment)
    {
        _environment = environment;
        this.function = function;
    }

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
        return _environment;
    }

    public void SetEnvironment(Environment environment1)
    {
        _environment = environment1;
    }

    public override string ToString()
    {
        return $"<Frame for function [{function}]>";
    }
}