namespace Rainbow.Memory;

public unsafe struct Block<T> where T: unmanaged
{
    public T* Raw { get; set; }
    public int Length { get; set; }

    public Block(T* ptr, int length)
    {
        Raw = ptr;
        Length = length;
    }
}