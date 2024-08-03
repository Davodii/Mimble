namespace VMProject;

public class CallFrame(Function function, Environment environment, int stackStart)
{
    public Environment Environment { get; } = environment;
    public Function Function { get; } = function;
    private int _ip = 0;
    private int StackStart { get; } = stackStart;

    public byte ReadByte()
    {
        return Function.Chunk.GetByte(_ip++);
    }
}