using Mimble.Exceptions;
using Mimble.Functions;
using Mimble.Values;
using ValueType = Mimble.Values.ValueType;

namespace Mimble;

public class Compiler
{
    // ! Parsing
    private Scanner _scanner = null!;
    private Token _previous = null!;
    private Token _current = null!;
    
    // ! Compiling
    private readonly Stack<UserDefined> _functions = new();
    private readonly Stack<LoopFrame> _loops = new();
    
    private UserDefined CurrentFunction()
    {
        return _functions.Peek();
    }
    
    #region Writing

    private void EmitByte(Instruction op)
    {
        // Write the op to the chunk
        CurrentFunction().GetChunk().Write((byte)op, _current.GetLine());
    }

    private void EmitByte(byte val)
    {
        CurrentFunction().GetChunk().Write(val, _current.GetLine());
    }

    private int EmitJump(Instruction op)
    {
        EmitByte(op);
        EmitByte(val:0);
        EmitByte(val:0);

        // Return index of the JumpIfFalse instruction
        return CurrentFunction().GetChunk().GetCodeCount() - 2;
    }

    private void PatchJump(int offset)
    {
        // Get the jump instruction 
        int jump = CurrentFunction().GetChunk().GetCodeCount() - offset - 2;
        
        // Store the jump in the next two bytes (after the jump instruction)
        CurrentFunction().GetChunk().Write(offset, (byte)((jump & 0xFF00) >> 8));
        CurrentFunction().GetChunk().Write(offset + 1, (byte)(jump & 0xFF));
    }

    private void EmitLoop(int loopStart)
    {
        EmitByte(Instruction.Loop);
        int offset = CurrentFunction().GetChunk().GetCodeCount() - loopStart + 2;

        EmitByte((byte)((offset & 0xFF00) >> 8));
        EmitByte((byte)(offset & 0xFF));
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
        else if (Match(TokenType.Continue))
        {
            if (_loops.Count == 0) throw new CompileTimeException(_previous, "Cannot 'continue' when not in a loop.");
            int jump = EmitJump(Instruction.Jump);
            _loops.Peek().Continues.Add(jump);
        }
        else if (Match(TokenType.Break))
        {
            if (_loops.Count == 0) throw new CompileTimeException(_previous, "Cannot 'break' when not in a loop.");
            int jump = EmitJump(Instruction.Jump);
            _loops.Peek().Breaks.Add(jump);
        }
        else if (Check(TokenType.Do))
        {
            BeginScope();
            Block();
            EndScope();
        }
        else
        {
            ExpressionStatement();
        }
    }
    
    private void FunctionCall()
    {
        // Previous token is the function identifier
        Token identifier = _previous;
        
        // Check the current identifier exists, or, add as constant
        int functionIndex;
        try
        {
            functionIndex = CurrentFunction().GetChunk().GetConstantIndex(identifier);
        }
        catch
        {
            functionIndex = CurrentFunction().GetChunk().AddConstant(new StringValue(identifier.GetValue()));
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
    
    private void ReturnStatement(bool exitMain = false)
    {
        if (_functions.Count == 1 && !exitMain)
        {
            // Inside the main function
            throw new CompileTimeException(_current, "Cannot return when not inside a function.");
        }

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
        EmitByte(Instruction.Pop);
        Consume(TokenType.Eol, "Expect expression to be terminated.");
    }

    private void FunctionDeclaration()
    {
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
                int index = newFunction.GetChunk().AddConstant(new IdentifierValue(identifier.GetValue()));
                EmitByte(Instruction.StoreVar);
                EmitByte((byte)index);
               
                Advance();
            } while (Match(TokenType.Comma));
        }

        newFunction.arity = argumentCount;
        Consume(TokenType.RightParen, "Expect ')' after parameters.");
        
        // ! Begin block
        Block(afterFunction:true);
        
        EmitByte(Instruction.Null);
        EmitByte(Instruction.Return);
        
        // add the function to the current chunk's constants
        //         // new Func(ValueType.UserDefinedFunction, newFunction,);new Func(ValueType.UserDefinedFunction, newFunction,);
        // new Func(ValueType.UserDefinedFunction, newFunction,);new Func(ValueType.UserDefinedFunction, newFunction,);

        Value functionValue = new FunctionValue(ValueType.UserDefinedFunction, newFunction);
        _functions.Pop();
        int functionIndex = CurrentFunction().GetChunk().AddConstant(functionValue);
        EmitByte(Instruction.DefFunction);
        EmitByte((byte)functionIndex);
    }

    private void WhileStatement()
    {
        BeginScope();
        int loopStart = CurrentFunction().GetChunk().GetCodeCount();

        LoopFrame frame = new LoopFrame();
        _loops.Push(frame);
        
        Expression();
        
        // Jump past body if expression is false
        int exitJump = EmitJump(Instruction.JumpIfFalse);
        EmitByte(Instruction.Pop);
        
        Block();

        // Patch continues
        foreach (var continueJump in frame.Continues)
        {
            PatchJump(continueJump);
        }
        
        // Loop back to the expression
        EmitLoop(loopStart);
        
        // Patch the jump
        PatchJump(exitJump);
        EmitByte(Instruction.Pop);
        
        // Path the breaks
        foreach (var breakJump in frame.Breaks)
        {
            PatchJump(breakJump);
        }
        
        EndScope();
    }

    private void ForStatement()
    {
        BeginScope();
        
        // Identifier
        Consume(TokenType.Identifier, "Expect identifier as looping variable.");
        int index; // index of the identifier inside the current chunk
        try
        {
            index = CurrentFunction().GetChunk().GetConstantIndex(_previous);
        }
        catch
        {
            // Add the constant to the constants list
            index = CurrentFunction().GetChunk().AddConstant(new StringValue(_previous.GetValue()));
        }
        
        Consume(TokenType.In, "Expect 'in' after expression.");

        // Array / list expression
        Expression();
        EmitByte(Instruction.CreateIterator);
        
        int loopStart = CurrentFunction().GetChunk().GetCodeCount();
        int forwardIterator = EmitJump(Instruction.ForwardIterator);
        
        LoopFrame frame = new LoopFrame();
        _loops.Push(frame);
        
        EmitByte(Instruction.StoreVar);
        EmitByte((byte)index);
        EmitByte(Instruction.Pop);
        
        Block();
        
        // Patch continues
        foreach (var continueJump in frame.Continues)
        {
            PatchJump(continueJump);
        }
        
        EmitLoop(loopStart);
        PatchJump(forwardIterator);
        EmitByte(Instruction.Pop);
        
        // Path the breaks
        foreach (var breakJump in frame.Breaks)
        {
            PatchJump(breakJump);
        }
        
        EndScope();
    }

    private void IfStatement()
    {
        Expression();
        
        // Start jump if false
        int jumpIf = EmitJump(Instruction.JumpIfFalse);
        // Pop the expression from the stack if the expression is true
        EmitByte(Instruction.Pop);
        
        BeginScope();
        Block();
        EndScope();
        
        int jumpToEnd = EmitJump(Instruction.Jump);
        
        // Jump to elif if false
        PatchJump(jumpIf);
        // Pop the expression from the stack
        EmitByte(Instruction.Pop);

        while (Match(TokenType.Elif))
        {
            Expression();
            jumpIf = EmitJump(Instruction.JumpIfFalse);
            EmitByte(Instruction.Pop);
            
            BeginScope();
            Block();
            EndScope();

            int nextJumpToEnd = EmitJump(Instruction.Jump);
            PatchJump(jumpIf);
            jumpToEnd = nextJumpToEnd;
        }

        if (Match(TokenType.Else))
        {
            BeginScope();
            Block();
            EndScope();
        }


        PatchJump(jumpToEnd);
    }

    private void Block(bool afterFunction = false)
    {
        // do ... end for normal 
        // does ... end for function definitions
        
        Consume(afterFunction ? TokenType.Does : TokenType.Do, "Expect block start.");
        
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
    private readonly Dictionary<TokenType, (bool, int)> _operatorInfo = new()
    {
        { TokenType.SqLeftBrace, (false, 0)},
        
        // Assignment
        { TokenType.Equal, (true, 0)},
        { TokenType.In, (false, 0)},
        
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

        int indexOfLhs = CurrentFunction().GetChunk().GetCodeCount();
        
        ComputeAtom();
        
        bool grouping = _previous.GetType() == TokenType.RightParen;
        
        // Continue until an operator of lower precedence than minPrecedence is found
        while (true)
        {
            if (!IsOperator(_current) || _operatorInfo[_current.GetType()].Item2 < minPrecedence) 
                break;
            
            // The current token is an operator
            
            // Get the precedence and associativity
            var (rightAssociative, precedence) = _operatorInfo[_current.GetType()];
            var op = _current;
            int nextMinPrecedence = rightAssociative ? precedence : precedence + 1;
            
            // Consume the current token and prepare the next one for the recursive call
            Advance();
            Expression(nextMinPrecedence);

            // Emit the correct bytes to the chunk
            HandleOperator(op, grouping, indexOfLhs);
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
            HandleIdentifier();
        }
        else if (Match(TokenType.SqLeftBrace))
        {
            // ! 
            HandleList();
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
            val = new NumberValue(asDouble);
        }
        else if (constant.GetType() == TokenType.String)
        {
            val = new StringValue(constant.GetValue());
        }
        else
        {
            throw new CompileTimeException(_current, "Current token is not a constant value.");
        }

        int index = CurrentFunction().GetChunk().AddConstant(val);
        
        EmitByte(Instruction.LoadConstant);
        EmitByte((byte)index);
    }

    private void HandleIdentifier()
    {
        // a
        // a[...]
        // a(...)

        Token identifier = _previous;
        
        // Get the index of the identifier inside the current chunk's constants
        int index;
        try
        {
            index = CurrentFunction().GetChunk().GetConstantIndex(identifier);
        }
        catch
        {
            // Add the constant to the constants list
            index = CurrentFunction().GetChunk().AddConstant(new StringValue(identifier.GetValue()));
        }
        
        if (Check(TokenType.LeftParen))
        {
            FunctionCall();
        }
        else
        {
            // Get the identifier and push onto the stack
            EmitByte(Instruction.LoadVar);
        
            // TODO: maybe introduce a LoadConstant 2x instruction where two bytes are used as the index
            EmitByte((byte)index);
        }
    }

    private void HandleList()
    {
        // Check if the list is defined as one of two things
        // [1..20:3] - range
        // [1,2,3,3] - definition
        
        if (Match(TokenType.SqRightBrace))
        {
            EmitByte(Instruction.CreateListFromValues);
            EmitByte((byte)0);
            return;
        }
        
        Expression();
        
        if (Match(TokenType.DoubleDot))
        {
            // ! Range
            Expression();

            if (Check(TokenType.Colon))
            {
                Expression();
                EmitByte(Instruction.CreateListFromRange);
            }
            else
            {
                // TODO: Maybe have a way to create automatically calculate increment
                // e.g. [1..3] -> [1..3:1]
                //      [1..-10] -> [1..-10:-1]
            }
        } 
        else
        {
            // ! Definition
            int itemCount = 1;

            while(Match(TokenType.Comma))
            {
                Expression();
                itemCount++;
            }
            
            Consume(TokenType.SqRightBrace, "Expect ']' after array items.");
            EmitByte(Instruction.CreateListFromValues);
            EmitByte((byte)itemCount);
            return;
        }
        
        Consume(TokenType.SqRightBrace, "Expect ']' after array definition.");
    }
    
    private void HandleOperator(Token token, bool grouping, int indexOfLhs)
    {
        TokenType op = token.GetType();
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
            case TokenType.In:
                // 
                EmitByte(Instruction.In);
                break;
            case TokenType.SqLeftBrace:
                // Consume the indexing part
                Consume(TokenType.SqRightBrace, "Expect ']' after array index.");
                if (Match(TokenType.Equal))
                {
                    // Assignment 
                    Expression();
                    EmitByte(Instruction.StoreSubscript);
                }
                else
                {
                    EmitByte(Instruction.GetSubscript);
                }
                break;
            case TokenType.Equal:
                // if (!identifier) throw new CompileTimeException(token, "Can only assign values to an identifier, not an expression.");
                if (CurrentFunction().GetChunk().GetByte(indexOfLhs) != (byte)Instruction.LoadVar || grouping)
                    throw new CompileTimeException(token,
                        "Can only assign values to an identifier, not an expression.");
                
                // Pop load instruction and index
                CurrentFunction().GetChunk().RemoveInstruction(indexOfLhs);
                byte identifierIndex = CurrentFunction().GetChunk().RemoveInstruction(indexOfLhs);
                
                // Add the store var
                EmitByte(Instruction.StoreVar);
                EmitByte(identifierIndex);
                break;
            default:
                throw new CompileTimeException(_current, "Unexpected token, expected operator.");
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
            case TokenType.Equal:       // =
            case TokenType.SqLeftBrace: // [
            case TokenType.In:          // in
                return true;
        }
        
        return false;
    }
    
    #endregion

    public UserDefined Compile(Scanner scanner)
    {
        _scanner = scanner;

        UserDefined main = new UserDefined("main", new Chunk());
        _functions.Push(main);
        
        // Parse the source text and compile to the chunk
        Advance();

        while (_current.GetType() != TokenType.Eof)
        {
            Declaration();
        }
        
        ReturnStatement(exitMain: true);
        
        return main;
    }
}