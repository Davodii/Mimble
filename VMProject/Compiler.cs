using System.Linq.Expressions;

namespace VMProject;

public class Compiler
{
    /*
     * Take source code tokens and compile into chunk(s) of bytecode
     */
    
    private Scanner _scanner; 
    private Token _previous;
    private Token _current;

    private void Advance() { 
        _previous = _current;
        
        for (;;) {
            _current = _scanner.ScanToken();
            
            //TODO: Handle errors from the scanner here
            break;
        }
    }

    public void Compile(char[] source)
    {
        // while not at the end of file 
        //      advance and compile
        
        // Set up the scanner
        Scanner scanner = new Scanner(source);
        
        
        
        Advance();

        for (;;)
        {
        }

        // Pass down any errors
    }
}