namespace VMProject;

public enum ValueType
{
    Null,
    Boolean,
    Number,
    String,
    Identifier,                 // ! used for variables
    UserDefinedFunction,
    NativeFunction,
    List,
}