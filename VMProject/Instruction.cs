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
    Jump,               // Increment ip by an offset
    JumpIfFalse,        // Increment ip by an offset if the value on the stack is false
    Loop,               // Decrement the ip by an offset    
    
    //TODO: handle arrays and other things idk
    // VARIABLES
    StoreVar,           // StoreVar [ VarIndex ]
    LoadVar,            // LoadVar [ VarIndex ]
    GetSubscript,
    StoreSubscript,
    DefFunction,
    
    CreateIterator,
    ForwardIterator,
    CreateListFromValues,       // CreateListFromValues [ Value count ]
    CreateListFromRange,        // CreateListFromRange  - top three values on the stack are: 1. step, 2. upper bound, 3. lower bound
    
    // BLOCKS
    BeginScope,
    EndScope,
    
    // FUNCTIONS
    Call,
    Return,
}