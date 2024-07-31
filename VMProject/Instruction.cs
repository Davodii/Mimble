namespace VMProject;

public enum Instruction
{
    // STACK
    Pop,
    
    // VALUES
    Null,
    False,
    True,
    Constant,
    
    // BINARY
    Equal,
    Greater,
    Less,
    Add,
    Subtract,
    Multiply,
    Divide,
    
    // UNARY
    Not,
    Negate,
    
    // JUMPS
    Jump,
    JumpIfFalse,
    Loop,
    
    //TODO: handle arrays and other things idk
    
    // FUNCTIONS
    Call,
    Return,
    
    End
}