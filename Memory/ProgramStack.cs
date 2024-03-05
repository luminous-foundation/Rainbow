namespace Rainbow.Memory;

public class ProgramStack
{
    public Block<byte> Memory { get; set; }
    public StackFrame Frame { get; set; }

    public ProgramStack(Block<byte> mem, StackFrame frame)
    {
        Memory = mem;
        Frame = frame;
    }
}

public class StackFrame
{
    private Block<byte> memory { get; set; }
    public int Start { get; set; }
    public int End { get; set; }
    public int StackPointer { get; set; }
    public StackLookup LookupTable { get; set; }

    public StackFrame(Block<byte> memory, int start, int end)
    {
        this.memory = memory;
        Start = start;
        End = end;

        StackPointer = start;
        LookupTable = new StackLookup();
    }

    public unsafe void Push<T>(ObjectInfo info, int size)
    {
        Block<byte> mem = new Block<byte>((byte *)(((long)memory.Raw) + StackPointer), size);
        info.Pointer = mem;

        StackPointer = StackPointer + size;
        
        LookupTable.Table.Add(info);
    }

    public ObjectInfo Pop()
    {
        StackPointer = StackPointer - LookupTable[^1].Pointer.Length;
        ObjectInfo ret = LookupTable[^1];
        LookupTable.Table.Remove(LookupTable[^1]);

        return ret;
    }
}

public class StackLookup
{
    public List<ObjectInfo> Table { get; set; } = new();
    public int Length => Table.Count;

    public ObjectInfo this[int index]
    {
        get { return Table[index]; }

        set { Table[index] = value; }
    }

    public ObjectInfo GetObjectByName(string name)
    {
        return Table.FirstOrDefault(x => x.Name == name) 
               ?? throw new NullReferenceException();
    }
}