namespace VMProject;

class Program
{
    private static string testString = "\"hello\" + 12\n";
    
    static void Main(string[] args)
    {
        Console.WriteLine(testString);
        Compiler compiler = new Compiler();

        Chunk chunk = compiler.Compile(testString);
        
        // chunk.PrintChunk();
        
        // Interpret the chunk
        VM vm = new VM();
        vm.Run(chunk);
    }
}