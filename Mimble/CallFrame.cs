using VMProject.Functions;

namespace VMProject;

public class CallFrame(UserDefined function, Environment environment)
{
    public UserDefined Function { get; } = function;
    private int _ip;

    public byte ReadByte()
    {
        return Function.Chunk.GetByte(_ip++);
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
        return $"<Frame for function [{Function}]>";
    }
}