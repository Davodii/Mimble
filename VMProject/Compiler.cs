using VMProject.Functions;

namespace VMProject;

public class Compiler
{
    // ! Parsing
    private Scanner _scanner = null!;
    private Token _previous = null!;
    private Token _current = null!;
    
    // ! Compiling
    private readonly Stack<UserDefined> _functions = new Stack<UserDefined>();
    
    private UserDefined CurrentFunction()
    {
        return _functions.Peek();
    }
    
    #region Writing

    private void EmitByte(Instruction op)
    {
        // Write the op to the chunk
        CurrentFunction().Chunk.Write((byte)op, _current.GetLine());
    }

    private void EmitByte(byte val)
    {
        CurrentFunction().Chunk.Write(val, _current.GetLine());
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
        if (_current.GetType() != type) throw new CompileTimeException(_current, msg);
        
        Advance();
    }
    #endregion
    
    #region Parser

    private void Declaration()
    {
        
        if (Match(TokenType.Function))
        {
            FunctionDeclaration();
        }
        else if (Match(TokenType.Eol))
        {
            // Empty line
            return;
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
        
        // TODO: check if we are accessing an index in the array 
        if (Check(TokenType.LeftParen))
        {
            FunctionCall();
        }

        else if (Check(TokenType.SqLeftBrace))
        {
            // Array initialization
        }
        
        // TODO: remove this function and incorporate into Expression()
        else if (Check(TokenType.Equal))
        {
            Assign();
        }
        
        Consume(TokenType.Eol, "Expect expression to be terminated.");
    }

    private void Assign()
    {
        Token identifier = _previous;
        int indexAsConstant = -1;
        try
        {
            indexAsConstant = CurrentFunction().Chunk.GetConstantIndex(identifier);
        }
        catch (CompileTimeException e)
        {
            // Add the identifier to the current chunk's constants
            indexAsConstant =
                CurrentFunction().Chunk.AddConstant(new Value(identifier.GetValue(), ValueType.Identifier));
        }
        
        Consume(TokenType.Equal, "Expect '=' after identifier.");
        
        // Assign statement
        Expression();
        EmitByte(Instruction.StoreVar);
        EmitByte((byte)indexAsConstant);
    }
    
    private void FunctionCall()
    {
        // Previous token is the function identifier
        Token identifier = _previous;
        
        // Check the current identifier exists, or, add as constant
        int functionIndex = -1;
        try
        {
            functionIndex = CurrentFunction().Chunk.GetConstantIndex(identifier);
        }
        catch
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

        if (Match(TokenType.Eol) || Match(TokenType.Eof))
        {
            EmitByte(Instruction.Null);
            EmitByte(Instruction.Return);
            return;
        }

        Expression();
        Consume(TokenType.Eol, "Expect only expression after 'return'.");
        EmitByte(Instruction.Return);
    }

    private void ExpressionStatement()
    {
        Expression();
        Consume(TokenType.Eol, "Expect expression to be terminated.");
    }

    private void FunctionDeclaration()
    {
        // Get the identifier
        // )
        // identifiers separated by colons
        // ) 
        // block
        
        Token functionIdentifier = _current;
        int argumentCount = 0;

        _functions.Push(new UserDefined(functionIdentifier.GetValue(), new Chunk()));
        UserDefined newFunction = _functions.Peek();
        
        Advance();
        Consume(TokenType.LeftParen, "Expect '(' after function declaration.");
        
        if (!Check(TokenType.RightParen))
        {
            do
            {
                argumentCount++;
                // ! Define variable to the function's constants
                Token identifier = _current;
                // Add the identifier to the function's constants
                int index = newFunction.Chunk.AddConstant(new Value(identifier.GetValue(), ValueType.Identifier));
                EmitByte(Instruction.StoreVar);
                EmitByte((byte)index);
               
                Advance();
            } while (Match(TokenType.Comma));
        }

        newFunction.Arity = argumentCount;
        Consume(TokenType.RightParen, "Expect ')' after parameters.");
        
        // ! Begin block
        Consume(TokenType.Does, "Expect 'does' after function parameter list.");
        Block();
        
        // TODO: make this better since there could already be a return inside the function
        EmitByte(Instruction.Null);
        EmitByte(Instruction.Return);
        
        // add the function to the current chunk's constants
        Value functionValue = new Value(newFunction, ValueType.UserDefinedFunction);
        _functions.Pop();
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

        /*if (Match(TokenType.Else))
        {
            Statement();
        }*/

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

    private void Block()
    {
        // do ... end for normal 
        // does ... end for function definitions
        Consume(TokenType.Eol, "Expect end of line after block start.");

        while (!Check(TokenType.End) && !Check(TokenType.Eof))
        {
            Declaration();
        }

        Consume(TokenType.End, "Unterminated block.");
        Consume(TokenType.Eol, "Expect end of line after 'end'.");
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
    private readonly Dictionary<TokenType, (bool, int)> _operatorInfo = new Dictionary<TokenType, (bool, int)>()
    {
        // Mul / Div
        { TokenType.Star, (false, 1)},  
        { TokenType.Slash, (false, 1)},
        
        // Add / Sub
        { TokenType.Plus, (false, 2)},
        { TokenType.Minus, (false,2)},
        
        // Relational
        { TokenType.Less, (false, 3)},
        { TokenType.LessEqual, (false, 3)},
        { TokenType.Greater, (false, 3)},
        { TokenType.GreaterEqual, (false, 3)},
        { TokenType.EqualEqual, (false, 3)},
        
        // Logical
        { TokenType.Or, (false, 4)},
        { TokenType.And, (false, 4)},
        { TokenType.Not, (true, 5)}
        
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
        }
    }

    private void ComputeAtom()
    {
        if (Match(TokenType.LeftParen))
        {
            Expression();
            Consume(TokenType.RightParen, "All groupings must be closed off.");
        }
        else if (Match(TokenType.Minus))
        {
            // ! Unary Minus
            HandleUnary(Instruction.Negate);
        }
        else if (Match(TokenType.Not))
        {
            // ! Unary not
            HandleUnary(Instruction.Not);
        }
        else if (Match(TokenType.False) || Match(TokenType.True) || Match(TokenType.Null))
        {
            // ! Literal
            HandleLiteral();
        }
        else if (Match(TokenType.Number) || Match(TokenType.String))
        {
            // ! Handle the constant 
            HandleConstant();
        }
        else if (Match(TokenType.Identifier))
        {
            // ! get local / global variable
            HandleIdentifier();
        }
        else
        {
            throw new CompileTimeException(_current, $"Unexpected token '{_current}'");
        }
    }

    private void HandleUnary(Instruction instruction)
    {
        ComputeAtom();
        EmitByte(instruction);
    }

    private void HandleLiteral()
    {
        // Handle false and true literals
        EmitByte(_previous.GetValue() == "false" ? Instruction.False : Instruction.True);
    }

    private void HandleConstant()
    {
        // Handle constants stored inside the chunk
        // Emit a byte to the stack 

        Token constant = _previous;
        Value val;
        
        if (constant.GetType() == TokenType.Number)
        {
            double asDouble = double.Parse(constant.GetValue());
            val = new Value(asDouble, ValueType.Number);
        }
        else if (constant.GetType() == TokenType.String)
        {
            val = new Value(constant.GetValue(), ValueType.String);
        }
        else
        {
            throw new CompileTimeException(_current, "Current token is not a constant value.");
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

        Token identifier = _previous;
        
        // TODO: compare between arrays, variables and functions
        
        // Get the index of the identifier inside the current chunk's constants
        int index = -1;

        try
        {
            index = CurrentFunction().Chunk.GetConstantIndex(identifier);
        }
        catch
        {
            // Add the constant to the constants list
            index = CurrentFunction().Chunk.AddConstant(new Value(identifier.GetValue(), ValueType.Identifier));
        }
        
        if (Check(TokenType.LeftParen))
        {
            FunctionCall();
        }
        else if (Check(TokenType.SqLeftBrace))
        {
            // array indexing 
        }
        else
        {
            // Get the identifier and push onto the stack
            EmitByte(Instruction.LoadVar);
        
            // TODO: maybe introduce a LoadConstant 2x instruction where two bytes are used as the index
            EmitByte((byte)index);
        }
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
            case TokenType.Not:
                EmitByte(Instruction.Not);
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
        }
    }
    
    private bool IsOperator(Token token)
    {
        // Check if the given token is an operator (either binary or unary)
        switch (token.GetType())
        {
            case TokenType.Minus:       // -
            case TokenType.Plus:        // +
            case TokenType.Slash:       // /
            case TokenType.Star:        // *
            case TokenType.And:         // and
            case TokenType.Or:          // or
            case TokenType.Not:         // not
            case TokenType.Less:        // <
            case TokenType.LessEqual:   // <=
            case TokenType.Greater:     // >
            case TokenType.GreaterEqual:// >=
            case TokenType.EqualEqual:  // ==
                return true;
        }
        
        return false;
    }
    
    #endregion

    public UserDefined Compile(string source)
    {
        // Initialize the scanner
        _scanner = new Scanner(source);

        UserDefined main = new UserDefined("main", new Chunk());
        _functions.Push(main);
        
        // Parse the source text and compile to the chunk
        Advance();

        while (_current.GetType() != TokenType.Eof)
        {
            Declaration();
        }
        
        ReturnStatement();
        
        return main;
    }
}