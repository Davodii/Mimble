namespace VMProject;

public class CallFrame(Function function, Environment environment)
{
    private Environment _environment  = environment;
    public Function Function { get; } = function;
    private int _ip = 0;

    public byte ReadByte()
    {
        return Function.Chunk.GetByte(_ip++);
    }

    public void AddOffset(int offset)
    {
        _ip += offset;
    }

    public Environment GetEnvironment()
    {
        return _environment;
    }

    public void SetEnvironment(Environment environment)
    {
        _environment = environment;
    }
}