using System.Runtime.Intrinsics.X86;

namespace VMProject;

class Program
{
    private static string testString = "a = 12\nb = a + 12\n";

    private static void REPL()
    {
        // begin running as if in a REPL

        VM vm = new VM();
        
        for(;;) 
        {
            Console.Write(" > ");
            string? input = Console.ReadLine();

            if (input == null || input == "exit") break;

            if (input[^1] != '\n')
                input += '\n';
            
            vm.Interpret(input);
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
        
        VM vm = new VM();
        vm.Interpret(source);
    }
    
    static void Main(string[] args)
    {
        if (args.Length == 0)
        {
            REPL();
        }
        else
        {
            FromFile(args[0]);
        }
    }
}