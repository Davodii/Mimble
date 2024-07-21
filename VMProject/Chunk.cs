namespace VMProject;

public struct Chunk
{
    // Store the array of bytecode instructions
    private List<byte> code;
    
    // Store constant values found within this chunk
    public List<Value> values;

    //TODO: Add some way to store the current line number
    // Use a delta-offset table
    // List of offset (from start of bytecode) and the delta change in the line number, can store it as a list of bytes
}