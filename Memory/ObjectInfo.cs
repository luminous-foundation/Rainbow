namespace Rainbow.Memory;

public class ObjectInfo
{
    public string Name { get; set; }
    public string Type { get; set; }
    public Block<byte> Pointer { get; set; }

    public ObjectInfo(string type, string name)
    {
        Type = type;
        Name = name;
    }
}