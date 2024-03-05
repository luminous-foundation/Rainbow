# INSTRUCTIONS

```
0x00	NOP
0x01-02	PUSH 	[imm/var]
0x03	POP		[var]
0x04-05	LDARG 	[imm/var]
0x06-07	CALL 	[func/var]
0x08-0C	ADD		[imm/var]	[imm/var]	[var]
0x0D-11	SUB		[imm/var]	[imm/var]	[var]
0x12-16	MUL		[imm/var]	[imm/var]	[var]
0x17-1B	DIV		[imm/var]	[imm/var]	[var]
0x1C-1D	JMP		[imm/var]
0x1E-26 JNE		[imm/var]	[imm/var]	[imm/var]
0x27-2F JE		[imm/var]	[imm/var]	[imm/var]
0x30-38 JGE		[imm/var]	[imm/var]	[imm/var]
0x39-41 JG		[imm/var]	[imm/var]	[imm/var]
0x42-4A JLE		[imm/var]	[imm/var]	[imm/var]
0x4B-53 JL		[imm/var]	[imm/var]	[imm/var]
0x54-55	MOV		[imm/var]	[var]
0x56-5A	AND		[imm/var]	[imm/var]	[var]
0x5B-5F	OR		[imm/var]	[imm/var]	[var]
0x60-64	XOR		[imm/var]	[imm/var]	[var]
0x65-69	NOT		[imm/var]	[var]
0x6A-6E	LSH		[imm/var]	[var]
0x6F-73	RSH		[imm/var]	[var]
0x74-75	VAR		[type/var]	[name]
```

# TYPES

```
0x00	void
0x01	i8
0x02	i16
0x03	i32
0x04	i64
0x05	u8
0x06	u16
0x07	u32
0x08	u64
0x09	f16
0x0A	f32
0x0B	f64
0x0C	pointer
0x0D	type
0x0E	struct
0x0F	bytecode string (used for variable names, function names, etc.)
```