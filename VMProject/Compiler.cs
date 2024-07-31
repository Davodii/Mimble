using System.Data.Common;
using System.Linq.Expressions;
using System.Reflection.Metadata;
using System.Reflection.Metadata.Ecma335;
using System.Runtime.Intrinsics.X86;
using System.Text.RegularExpressions;

namespace VMProject;

public class Compiler
{
    /*
     * Use an LALR parser to parse the input and create the necessary bytecode 
     */

    private Scanner _scanner = null;
    private Token _previous = null;
    private Token _current = null;
    private Chunk _currentChunk = null;
    private List<Token> _identifiers;
    
    
    #region Chunk

    private void EmitByte(Instruction op)
    {
        // Write the op to the chunk
        _currentChunk.Write((byte)op);
    }

    private void EmitByte(byte val)
    {
        _currentChunk.Write(val);
    }

    private int EmitJump(Instruction op)
    {
        // JumpIfFalse [ Instruction to jump to ]
        
        EmitByte(op);
        EmitByte(val:0);
        EmitByte(val:0);

        // Return index of the JumpIfFalse instruction
        return _currentChunk.GetCodeCount() - 2;
    }

    private void PatchJump(int offset)
    {
        // Get the jump instruction 
        int jump = _currentChunk.GetCodeCount() - offset - 2;
        
        // Store the jump in the next two bytes (after the jump instruction)
        _currentChunk.Write((byte)((jump & 0xFF00) >> 8), offset);
        _currentChunk.Write((byte)(jump & 0xFF), offset + 1);
    }
    
    #endregion
    
    #region Tokens
    private void Advance()
    {
        _previous = _current;
        _current = _scanner.ScanToken();
        
        //TODO: handle error tokens (enter error recovery mode)
    }

    private bool Check(TokenType type)
    {
        return _current.GetType() == type;
    }

    private bool Match(TokenType type)
    {
        if (!Check(type)) return false;
        Advance();
        return true;
    }

    private void Consume(TokenType type, string msg)
    {
        if (_current.GetType() == type)
        {
            Advance();
            return;
        }
        
        //TODO: throw error with the given msg
    }
    #endregion
    
    #region Parser

    private void Declaration()
    {
        
        if (Match(TokenType.Function))
        {
            FunctionDeclaration();
        }
        else
        {
            Statement();
        }
    }

    private void Statement()
    {
        /*
         * statement: expression_statement
                    | if_statement
                    | for_statement
                    | while_statement
                    | return_statement
                    | block
         */
        if (Match(TokenType.If))
        {
            IfStatement();
        } 
        else if (Match(TokenType.For))
        {
            ForStatement();
        } 
        else if (Match(TokenType.While))
        {
            WhileStatement();
        } 
        else if (Match(TokenType.Return))
        {
            ReturnStatement();
        }
        else if (Match(TokenType.Do))
        {
            Block();
        }
        else if (Match(TokenType.Identifier))
        {
            AssignStatement();
            
        }
        else
        {
            ExpressionStatement();
        }
    }

    private void AssignStatement()
    {
        //TODO: change the signature of this function, it irks me
        
        // ! Identifier
        // a = ...      - Variable Initialization
        // a[i] = ...   - Array indexing and initialization 

        // TODO: Remove everything here and make pretty / better
        Token identifier = _previous;
        Value identifierValue = new Value(identifier.GetValue(), ValueType.VariableIdentifier);

        int localIndex = _currentChunk.GetLocalIndex(identifierValue);

        if (localIndex == -1)
        {
            localIndex = _currentChunk.AddLocal(identifierValue);
        }
        
        // Handle everything after the identifier
        if (Match(TokenType.SqLeftBrace))
        {
            // Array accessing
            
            // Match an equal after the end of the array indexingf
        }
        else if (Match(TokenType.Equal))
        {
            // Initialize
            Expression();
            EmitByte(Instruction.StoreVar);
            EmitByte((byte)localIndex);
        }
    }
    
    private void ReturnStatement()
    {
        //TODO: check not at the top level of the program and 
        //      actually inside of a function

        if (Match(TokenType.Eol))
        {
            //TODO: write null to the return value
            //      emit return
            EmitByte(Instruction.Null);
            EmitByte(Instruction.Return);
        }

        Expression();
        Consume(TokenType.Eol, "Expect only expression after 'return'.");
        EmitByte(Instruction.Return);
    }

    private void ExpressionStatement()
    {
        Expression();
        
        //TODO: check no other tokens (i.e. \n found)
        
        // EmitByte(Instruction.Pop);
    }

    private void FunctionDeclaration()
    {
        return;
        // Get the identifier
        // )
        // identifiers separated by colons
        // ) 
        // block
    }

    private void WhileStatement()
    {
        // expression
        // block
        
        Expression();
        EmitByte(Instruction.Pop);
        
        // Jump
        
        Statement();
        
        // Jump to the expression
    }

    private void ForStatement()
    {
        return;
        // throw new NotImplementedException();
        // identifier
        // in
        // expression ??? 
        // block

        //Identifier();

        //Match(TokenType.In);

        //Expression();

        //Block();
    }

    private void IfStatement()
    {
        Expression();
        
        // Start jump if false
        int jumpIf = EmitJump(Instruction.JumpIfFalse);
        // Pop the expression from the stack
        EmitByte(Instruction.Pop);
        
        Statement();

        int elseJump = EmitJump(Instruction.Jump);
        
        // Jump to elif if false
        PatchJump(jumpIf);
        // Pop the expression from the stack
        EmitByte(Instruction.Pop);
        
        // TODO: implement Elif statements
        // if (Check(TokenType.Elif))
        // {
        //     ElifStatements();
        // }

        if (Match(TokenType.Else))
        {
            Statement();
        }

        PatchJump(elseJump);
    }

    private void ElifStatements()
    {
        // elif expression block
        // elif statements
        Expression();
        
        // Jump past statement if false
        int jump = EmitJump(Instruction.JumpIfFalse);
        EmitByte(Instruction.Pop);
        
        Statement();
        
        // Check if there are more elif statements
        if (Check(TokenType.Elif))
        {
            ElifStatements();
        }
        
        // Should be good rn
    }

    private void Block(bool afterFunctionDefinition = false)
    {
        // do ... end for normal 
        // does ... end for function definitions
        Match(afterFunctionDefinition ? TokenType.Does : TokenType.Do);
        
        //TODO: start a new scope for the variables
        
        Statement();
        
        //TODO: end the scope
    }
    
    #endregion
    
    #region Expressions
    

    private Dictionary<TokenType, (bool, int)> _operatorInfo = new Dictionary<TokenType, (bool, int)>()
    {
        { TokenType.Or, (false, 0)},
        { TokenType.And, (false, 1)},
        { TokenType.Plus, (false, 2)},
        { TokenType.Minus, (false,2)},
        { TokenType.Star, (false, 3)},
        { TokenType.Slash, (false, 3)},
        { TokenType.Not, (true, 4)}
    };
    
    private void Expression(int minPrecedence = 0)
    {
        // Handle all expression
        // as well as variable declaration
        
        // Compute the left hand side of the expression
        // Push the result(s) to the stack  
        ComputeAtom();
        
        // Continue until an operator of lower precedence than minPrec is found
        while (true)
        {
            if (!IsOperator(_current) || _operatorInfo[_current.GetType()].Item2 < minPrecedence) 
                break;
            
            // The current token is an operator
            
            // Get the precedence and associativity
            var opInfoTuple = _operatorInfo[_current.GetType()];
            var op = _current.GetType();
            int precedence = opInfoTuple.Item2;
            bool rightAssociative = opInfoTuple.Item1;
            int nextMinPrecedence = rightAssociative ? precedence : precedence + 1;
            
            // Consume the current token and prepare the next one for the recursive call
            Advance();
            Expression(nextMinPrecedence);

            // Emit the correct bytes to the chunk
            HandleOperator(op);
            
            break;
        }
    }

    private void ComputeAtom()
    {
        if (_current.GetType() == TokenType.LeftParen)
        {
            Advance();
            Expression();
            
            Consume(TokenType.RightParen, "All groupings must be closed off.");
            
            Advance();
            return;
        }

        if (_current.GetType() == TokenType.Minus)
        {
            // ! Unary Minus
            Unary(Instruction.Negate);
            return;
        }

        if (_current.GetType() == TokenType.Not)
        {
            // ! Unary not
            Unary(Instruction.Not);
            return;
        }
        
        if (_current.GetType() == TokenType.False || _current.GetType() == TokenType.True || 
            _current.GetType() == TokenType.Null)
        {
            // ! literal
            Literal();
        }
        else if (_current.GetType() == TokenType.Number || _current.GetType() == TokenType.String)
        {
            // ! Handle the constant 
            Constant();
        }
        else if (_current.GetType() == TokenType.Identifier)
        {
            // ! get local / global variable
            Identifier();
        }
        else
        {
            // ! Unexpected token here / unterminated expression
            Console.WriteLine("Unexpected token type: " + _current.GetType());
        }
        
        Advance();
    }

    private void Unary(Instruction instruction)
    {
        Advance();
        ComputeAtom();
        EmitByte(instruction);
    }

    private void Literal()
    {
        // Handle false and true literals
        if (_current.GetValue() == "false")
        {
            EmitByte(Instruction.False);
        }
        else 
        {
            EmitByte(Instruction.True);
        }
        
        Advance();
    }

    private void Constant()
    {
        // Handle constants stored inside the chunk
        // Emit a byte to the stack 

        Value val = new Value(null, ValueType.Null);
        
        // Current is a constant value
        if (_current.GetType() == TokenType.Number)
        {
            double asDouble = double.Parse(_current.GetValue());
            val = new Value(asDouble, ValueType.Number);
        }
        else if (_current.GetType() == TokenType.String)
        {
            val = new Value(_current.GetValue(), ValueType.String);
        }

        int valIndex = _currentChunk.AddLocal(val);
        
        EmitByte(Instruction.LoadConstant);
        EmitByte((byte)valIndex);
    }

    private void Identifier()
    {
        //TODO: handle identifiers inside of an expression 
        
        // a
        // a[...]
        // a(...)
    }

    private void HandleOperator(TokenType op)
    {
        switch (op)
        {
            case TokenType.Plus:
                EmitByte(Instruction.Add);
                break;
            case TokenType.Minus:
                EmitByte(Instruction.Subtract);
                break;
            case TokenType.Star:
                EmitByte(Instruction.Multiply);
                break;
            case TokenType.Slash:
                EmitByte(Instruction.Divide);
                break;
            case TokenType.And:
                EmitByte(Instruction.And);
                break;
            case TokenType.Or:
                EmitByte(Instruction.Or);
                break;
            case TokenType.EqualEqual:
                EmitByte(Instruction.Equal);
                break;
            case TokenType.Greater:
                EmitByte(Instruction.Greater);
                break;
            case TokenType.GreaterEqual:
                EmitByte(Instruction.Less);
                EmitByte(Instruction.Not);
                break;
            case TokenType.Less:
                EmitByte(Instruction.Less);
                break;
            case TokenType.LessEqual:
                EmitByte(Instruction.Greater);
                EmitByte(Instruction.Not);
                break;
            default:
                // ! Should never be here
                break;
        }
    }
    
    private bool IsOperator(Token token)
    {
        // Check if the given token is an operator (either binary or unary)
        switch (token.GetType())
        {
            case TokenType.Minus:
            case TokenType.Plus:
            case TokenType.Slash:
            case TokenType.Star:
            case TokenType.And:
            case TokenType.Or:
            case TokenType.Not:
                return true;
        }
        
        return false;
    }
    
    #endregion

    public Chunk Compile(string source)
    {
        // Initialize the scanner
        _scanner = new Scanner(source);
        
        // Initialize the bytecode chunk
        _currentChunk = new Chunk();
        
        //TODO: Remove this
        _identifiers = new List<Token>();
        
        // Parse the source text and compile to the chunk
        Advance();
        
        Declaration();
        
        EmitByte(Instruction.End);
        
        return _currentChunk;
    }
}