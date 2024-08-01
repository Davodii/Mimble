namespace VMProject;

public class CallFrame(Function function, Environment environment)
{
    private Environment _environment = environment;

    private Function _function = function;

    private int _ip = 0;
}