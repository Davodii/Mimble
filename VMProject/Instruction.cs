namespace VMProject;

public enum Instruction
{
    // STACK
    Pop,
    
    // VALUES
    Null,
    False,
    True,
    LoadConstant,       // LoadConst [ ConstIndex ]
    
    // BINARY
    And,
    Or,
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
    // VARIABLES
    StoreVar,           // StoreVar [ VarIndex ]
    LoadVar,            // LoadVar [ VarIndex ]
    
    
    // FUNCTIONS
    Call,
    Return,
    
    //TODO: remove this v
    End
}