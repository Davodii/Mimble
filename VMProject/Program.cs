namespace VMProject;

class Program
{
    private static string testString = "if true a = 12\n";
    
    static void Main(string[] args)
    {
        Console.WriteLine(testString);
        Compiler compiler = new Compiler();

        Function main = compiler.Compile(testString);
        Console.WriteLine("Lets get to printing");
        main.Chunk.PrintChunk();
    }
}