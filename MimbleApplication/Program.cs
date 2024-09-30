namespace Mimble;

public static class Program
{
    private static Interpreter _interpreter;
    
    private static void Repl()
    {
        // begin running as if in a REPL
        
        for(;;) 
        {
            Console.Write(" > ");
            string? input = Console.ReadLine();

            if (input == null || input == "exit") break;

            if (input[^1] != '\n')
                input += '\n';
            
            _interpreter.Interpret(input);
        }
    }

    private static void FromFile(string filePath)
    {
        // Get the file as source
        StreamReader reader = new StreamReader(filePath);
        string source = reader.ReadToEnd();

        if (source[^1] != '\n')
        {
            source += '\n';
        }
        
        _interpreter.Interpret(source);
    }
    
    static void Main(string[] args)
    {
        _interpreter = new Interpreter();
        
        if (args.Length == 0)
        {
            Repl();
        }
        else
        {
            FromFile(args[0]);
        }
    }
}