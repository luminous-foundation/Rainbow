# RESERVED BYTES
```
0xFF - function start
0xFE - scope start
0xFD - scope end
```

# INSTRUCTIONS

```
[x] 0x00	NOP
[x] 0x01-02	PUSH    [imm/var]
[x] 0x03	POP     [var]
[ ] 0x04-05	LDARG   [imm/var]
[ ] 0x06-07	CALL    [func/var]
[x] 0x08-0B	ADD     [imm/var]   [imm/var]   [var]
[x] 0x0C-0F	SUB     [imm/var]   [imm/var]   [var]
[x] 0x10-13	MUL     [imm/var]   [imm/var]   [var]
[x] 0x14-17	DIV     [imm/var]   [imm/var]   [var]
[ ] 0x19-1A	JMP     [imm/var]
[ ] 0x1B-22 JNE     [imm/var]   [imm/var]   [imm/var]
[ ] 0x23-2A JE      [imm/var]   [imm/var]   [imm/var]
[ ] 0x2B-32 JGE     [imm/var]   [imm/var]   [imm/var]
[ ] 0x33-3A JG      [imm/var]   [imm/var]   [imm/var]
[ ] 0x3B-42 JLE     [imm/var]   [imm/var]   [imm/var]
[ ] 0x43-4A JL      [imm/var]   [imm/var]   [imm/var]
[ ] 0x4B-4C	MOV     [imm/var]   [var]
[ ] 0x4D-50	AND     [imm/var]   [imm/var]   [var]
[ ] 0x51-54	OR      [imm/var]   [imm/var]   [var]
[ ] 0x60-58	XOR     [imm/var]   [imm/var]   [var]
[ ] 0x59-5C	NOT     [imm/var]   [var]
[ ] 0x6A-60	LSH     [imm/var]   [var]
[ ] 0x6F-64	RSH     [imm/var]   [var]
[ ] 0x74-68	VAR     [type/var]  [name]
[ ] 0x76-77 RET     [imm/var]
```

# TYPES

```
0x00	void
0x01	i8
0x02	i16
0x03	i32
0x04	i64
0x05	u8/char
0x06	u16
0x07	u32
0x08	u64
0x09	f16
0x0A	f32
0x0B	f64
0x0C	pointer
0x0D	type
0x0E	struct
0x0F	bytecode string (used for variable names, function names, etc.) (also is a function pointer)
```

# BYTECODE STRINGS
Bytecode strings are how function names/variable names/etc. are represented in the bytecode.
Their format is a follows
```
(1 byte length) (characters)
```
An example string would look like this:
```
0D 48 65 6C 6C 6F 2C 20 57 6F 72 6C 64 21
```
This as text is
```
Hello, World!
```

# FUNCTIONS
The format for functions is as follows
```
FF (return type) (function name) (args) FE
    (code)
FD
```
An example function would look like this:
```
FF 01 04 6D 61 69 6E 0C 0C 05 04 61 72 67 73 FE
    (code)
FD
```
In pseudocode this would be
```cs
byte main(string[] args) {

}
```