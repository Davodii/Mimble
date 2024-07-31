namespace VMProject;

public enum ValueType
{
    Null,
    Boolean,
    Number,
    String,
    VariableIdentifier,         // ! used for variables
    Object      // Will be used for things like arrays and other values
}