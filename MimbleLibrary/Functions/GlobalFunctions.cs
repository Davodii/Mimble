namespace Mimble.Functions;

[Flags]
public enum GlobalFunctions
{
    None = 0,
    IO = 1 << 0,
    Lists = 1 << 1,
    
    
    // ! TODO: update this enum to include more global functions
    All = IO | Lists
}