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

    // ! Parsing
    private Scanner _scanner = null;
    private Token _previous = null;
    private Token _current = null;
    
    // ! Compiling
    private Function a;

    private Stack<Function> _functions;

    private Function CurrentFunction()
    {
        return _functions.Peek();
    }
    
    #region Writing

    private void EmitByte(Instruction op)
    {
        // Write the op to the chunk
        CurrentFunction().Chunk.Write((byte)op);
    }

    private void EmitByte(byte val)
    {
        CurrentFunction().Chunk.Write(val);
    }

    private int EmitJump(Instruction op)
    {
        EmitByte(op);
        EmitByte(val:0);
        EmitByte(val:0);

        // Return index of the JumpIfFalse instruction
        return CurrentFunction().Chunk.GetCodeCount() - 2;
    }

    private void PatchJump(int offset)
    {
        // Get the jump instruction 
        int jump = CurrentFunction().Chunk.GetCodeCount() - offset - 2;
        
        // Store the jump in the next two bytes (after the jump instruction)
        CurrentFunction().Chunk.Write(offset, (byte)((jump & 0xFF00) >> 8));
        CurrentFunction().Chunk.Write(offset + 1, (byte)(jump & 0xFF));
    }

    private void EmitLoop(int loopStart)
    {
        EmitByte(Instruction.Loop);
        int offset = CurrentFunction().Chunk.GetCodeCount() - loopStart + 2;
        
        EmitByte((byte)((offset & 0xFF00) >> 8));
        EmitByte((byte)((offset & 0xFF)));
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
            BeginScope();
            Block();
            EndScope();
        }
        else if (Match(TokenType.Identifier))
        {
            Identifier();
        }
        else
        {
            ExpressionStatement();
        }
    }

    private void Identifier()
    {
        //TODO: change the signature of this function, it irks me
        
        // ! Identifier
        // a = ...      - Variable Initialization
        // a[i] = ...   - Array indexing and initialization 
        // Function call

        // Get the identifier as a string
        // Store the string in the current chunk's constants value

        Token identifier = _current;
        int indexAsConstant = CurrentFunction().Chunk.GetConstantIndex(identifier);

        if (indexAsConstant == -1)
        {
            // Add the identifier to the current chunk's constants
            indexAsConstant =
                CurrentFunction().Chunk.AddConstant(new Value(identifier.GetValue(), ValueType.Identifier));
        }

        // TODO: check if we are accessing an index in the array 
        if (Check(TokenType.LeftParen))
        {
            // Function call
            // TODO: create a function to handle function calls for here and inside HandleIdentifier()
            
            Advance();
            return;
        }

        if (Match(TokenType.SqLeftBrace))
        {
            // Array initialization
        }
        
        if (Match(TokenType.Equal))
        {
            // Assign statement
            Expression();
            EmitByte(Instruction.StoreVar);
            EmitByte((byte)indexAsConstant);
        }
        
        Advance();
    }

    private void FunctionCall()
    {
        // Current token is the function identifier
        Token identifier = _current;
        
        // Check the current identifier exists, or, add as constant
        int functionIndex = CurrentFunction().Chunk.GetConstantIndex(identifier);
        if (functionIndex == -1)
        {
            functionIndex = CurrentFunction().Chunk.AddConstant(new Value(identifier.GetValue(), ValueType.Identifier));
        }
        
        Consume(TokenType.LeftParen, "Expect '(' after function identifier.");
        
        // Create a new function object
        int argumentCount = 0;
        
        if (!Check(TokenType.RightParen))
        {
            // Skip expressions and all that
            do
            {
                // Compute expression
                Expression();
                argumentCount++;
            } while (Match(TokenType.Comma));
        }
        
        Consume(TokenType.RightParen, "Expect ')' after function call.");
        
        // Load the function onto the stack
        EmitByte(Instruction.LoadVar);
        EmitByte((byte)functionIndex);
        
        // Execute the function
        EmitByte(Instruction.Call);
        EmitByte((byte)argumentCount);
        
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
        // Get the identifier
        // )
        // identifiers separated by colons
        // ) 
        // block
        
        Token identifier = _current;
        int argumentCount = 0;

        _functions.Push(new Function(identifier.GetValue(), new Chunk()));
        Function newFunction = _functions.Peek();
        
        // ! Begin a new scope
        // ! Might not need to do this for here since a function has its own environmnet anyway
        // BeginScope();
        
        Consume(TokenType.LeftParen, "Expect '(' after function declaration.");
        
        if (!Check(TokenType.RightParen))
        {
            do
            {
                argumentCount++;
                // ! Define variable to the function's constants
                // Identifier();
            } while (Match(TokenType.Comma));
        }

        newFunction.Arity = argumentCount;
        Consume(TokenType.RightParen, "Expect ')' after parameters.");
        
        // ! Begin block
        Block(true);
        
        // Handle function code and what not
        // EndScope();
        
        // add the function to the current chunk's constants
        Value functionValue = new Value(_functions.Pop(), ValueType.Function);
        int functionIndex = CurrentFunction().Chunk.AddConstant(functionValue);
        EmitByte(Instruction.DefFunction);
        EmitByte((byte)functionIndex);
    }

    private void WhileStatement()
    {

        int loopStart = CurrentFunction().Chunk.GetCodeCount();
        Expression();
        
        // Jump past body if expression is false
        int exitJump = EmitJump(Instruction.JumpIfFalse);
        EmitByte(Instruction.Pop);
        
        Statement();
        
        // Loop back to the expression
        EmitLoop(loopStart);
        
        // Patch the jump
        PatchJump(exitJump);
        EmitByte(Instruction.Pop);
        
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
        // Pop the expression from the stack if the expression is true
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
        
        Statement();
        
        Match(TokenType.End);
    }

    private void BeginScope()
    {
        EmitByte(Instruction.BeginScope);
    }

    private void EndScope()
    {
        EmitByte(Instruction.EndScope);
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
            HandleUnary(Instruction.Negate);
            return;
        }

        if (_current.GetType() == TokenType.Not)
        {
            // ! Unary not
            HandleUnary(Instruction.Not);
            return;
        }
        
        if (_current.GetType() == TokenType.False || _current.GetType() == TokenType.True || 
            _current.GetType() == TokenType.Null)
        {
            // ! literal
            HandleLiteral();
        }
        else if (_current.GetType() == TokenType.Number || _current.GetType() == TokenType.String)
        {
            // ! Handle the constant 
            HandleConstant();
        }
        else if (_current.GetType() == TokenType.Identifier)
        {
            // ! get local / global variable
            HandleIdentifier();
        }
        else
        {
            // ! Unexpected token here / unterminated expression
            Console.WriteLine("Unexpected token type: " + _current.GetType());
        }
        
        Advance();
    }

    private void HandleUnary(Instruction instruction)
    {
        Advance();
        ComputeAtom();
        EmitByte(instruction);
    }

    private void HandleLiteral()
    {
        // Handle false and true literals
        EmitByte(_current.GetValue() == "false" ? Instruction.False : Instruction.True);
    }

    private void HandleConstant()
    {
        // Handle constants stored inside the chunk
        // Emit a byte to the stack 

        Value val;
        
        if (_current.GetType() == TokenType.Number)
        {
            double asDouble = double.Parse(_current.GetValue());
            val = new Value(asDouble, ValueType.Number);
        }
        else if (_current.GetType() == TokenType.String)
        {
            val = new Value(_current.GetValue(), ValueType.String);
        }
        else
        {
            // ! Error
            return;
        }

        int index = CurrentFunction().Chunk.AddConstant(val);
        
        EmitByte(Instruction.LoadConstant);
        EmitByte((byte)index);
    }

    private void HandleIdentifier()
    {
        //TODO: handle identifiers inside of an expression 
        
        // a
        // a[...]
        // a(...)

        Token identifier = _current;
        
        // TODO: compare between arrays, variables and functions
        
        // Get the index of the identifier inside the current chunk's constants
        int index = CurrentFunction().Chunk.GetConstantIndex(identifier);

        if (index == -1)
        {
            // Add the constant to the constants list
            index = CurrentFunction().Chunk.AddConstant(new Value(identifier.GetValue(), ValueType.Identifier));
        }
        
        // Get the identifier and push onto the stack
        EmitByte(Instruction.LoadVar);
        
        // TODO: maybe introduce a LoadConstant 2x instruction where two bytes are used as the index
        EmitByte((byte)index);
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

    public Function Compile(string source)
    {
        // Initialize the scanner
        _scanner = new Scanner(source);

        Function main = new Function("main", new Chunk());
        
        // Parse the source text and compile to the chunk
        Advance();
        
        Declaration();
        
        ReturnStatement();
        
        return main;
    }
}