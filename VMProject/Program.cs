namespace VMProject;

class Program
{
    private static string testString = "if true a = 12\n";
    
    static void Main(string[] args)
    {
        Console.WriteLine(testString);
        Compiler compiler = new Compiler();

        Chunk chunk = compiler.Compile(testString);
        Console.WriteLine("Lets get to printing");
        chunk.PrintChunk();
        
        // Interpret the chunk
        VM vm = new VM();
        vm.Run(chunk);
    }
}