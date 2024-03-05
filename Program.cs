using Rainbow.Memory;

class Program { 
    public static unsafe void Main(String[] args)
    {
        byte *stackMem = stackalloc byte[500 * 1024];
        Block<byte> mem = new Block<byte>(stackMem, 500 * 1024);
        ProgramStack stack = new ProgramStack(mem, new StackFrame(mem, 0, 2047));
        stack.Frame.Push<byte>(new ObjectInfo("balls", "testicles"), sizeof(int));

        Console.WriteLine(stack.Frame.Start);
        Console.WriteLine(stack.Frame.StackPointer);
        
        stack.Frame.Pop();
        Console.WriteLine(stack.Frame.StackPointer);
        
        byte[] program = File.ReadAllBytes(args[0]);

        for(int i  = 0; i < program.Length; i++) {
            Console.WriteLine(program[i]);
        }
    }
}