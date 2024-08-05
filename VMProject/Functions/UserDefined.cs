namespace VMProject.Functions;

public class UserDefined(string identifier, Chunk chunk) : Function(identifier)
{
    public Chunk Chunk { get;  } = chunk;
}