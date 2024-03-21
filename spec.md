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
[x] 0x04-05	LDARG   [imm/var]
[ ] 0x06-07	CALL    [func/var]
[x] 0x08-0B	ADD     [imm/var]   [imm/var]   [var]
[x] 0x0C-0F	SUB     [imm/var]   [imm/var]   [var]
[x] 0x10-13	MUL     [imm/var]   [imm/var]   [var]
[x] 0x14-17	DIV     [imm/var]   [imm/var]   [var]
[x] 0x18-19	JMP     [imm/var]
[x] 0x1A-21 JNE     [imm/var]   [imm/var]   [imm/var]
[x] 0x22-29 JE      [imm/var]   [imm/var]   [imm/var]
[x] 0x2A-31 JGE     [imm/var]   [imm/var]   [imm/var]
[x] 0x32-39 JG      [imm/var]   [imm/var]   [imm/var]
[x] 0x3A-41 JLE     [imm/var]   [imm/var]   [imm/var]
[x] 0x42-49 JL      [imm/var]   [imm/var]   [imm/var]
[x] 0x4A-4B	MOV     [imm/var]   [var]
[ ] 0x4C-4F	AND     [imm/var]   [imm/var]   [var]
[ ] 0x50-53	OR      [imm/var]   [imm/var]   [var]
[ ] 0x54-58	XOR     [imm/var]   [imm/var]   [var]
[ ] 0x59-5A	NOT     [imm/var]   [var]
[ ] 0x5B-5C	LSH     [imm/var]   [var]
[ ] 0x5D-5E	RSH     [imm/var]   [var]
[x] 0x5F-60	VAR     [type/var]  [name]
[ ] 0x61-62 RET     [imm/var]
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