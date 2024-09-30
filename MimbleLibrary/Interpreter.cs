using Mimble.Functions;

namespace Mimble;

public class Interpreter
{
    // Setup a Compiler, VM and include any functions the user passes in

    private readonly Scanner _scanner = new();
    private readonly Compiler _compiler = new();
    private readonly VM _vm = new();
    private Environment? _global;


    public void GenerateGlobalEnvironment(GlobalFunctions functions)
    {
        _global = GlobalScope.CreateGlobal(functions);
    }
    
    public void GenerateGlobalEnvironment(GlobalFunctions functions, Environment custom)
    {
        _global = new Environment(GlobalScope.CreateGlobal(functions));

        foreach (var pair in custom.GetLocals())
        {
            _global.Assign(pair.Key, pair.Value);
        }
    }

    public void Interpret(string source)
    {
        if (_global == null)
        {
             GenerateGlobalEnvironment(GlobalFunctions.All);
        }
        
        _scanner.SetSource(source);
        UserDefined main = _compiler.Compile(_scanner);
        
        // _global will be defined here
        _vm.Run(main, _global!);
    }
    
}