class Program { 
    public static void Main(String[] args) {
        byte[] program = File.ReadAllBytes(args[0]);

        for(int i  = 0; i < program.Length; i++) {
            Console.WriteLine(program[i]);
        }
    }
}