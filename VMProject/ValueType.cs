namespace VMProject;

public enum ValueType
{
    Null,
    Boolean,
    Number,
    String,
    Identifier,         // ! used for variables
    Function,
    Object                      // Will be used for things like arrays and other values
}