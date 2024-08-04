namespace VMProject;

class Program
{
    private static string testString = "a = 12\nb = a + 12\n";
    
    static void Main(string[] args)
    {
        VM vm = new VM();
        
        vm.Interpret(testString);
    }
}