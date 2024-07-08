# RAINBOW BYTECODE SPECIFICATION

## RESERVED BYTES
```
0xFF - function start
0xFE - scope start
0xFD - scope end
0xFC - data section start
0xFB - struct start
```

## INSTRUCTIONS

```
[ ] 0x00        NOP
[ ] 0x01-02     PUSH    [imm/var]
[ ] 0x03        POP     [var]
[ ] 0x04-05     LDARG   [imm/var]
[ ] 0x06-07     CALL    [func/var]
[ ] 0x08-0B     ADD     [imm/var]   [imm/var]   [var]
[ ] 0x0C-0F     SUB     [imm/var]   [imm/var]   [var]
[ ] 0x10-13     MUL     [imm/var]   [imm/var]   [var]
[ ] 0x14-17     DIV     [imm/var]   [imm/var]   [var]
[ ] 0x18-19     JMP     [imm/var]
[ ] 0x1A-21     JNE     [imm/var]   [imm/var]   [imm/var]
[ ] 0x22-29     JE      [imm/var]   [imm/var]   [imm/var]
[ ] 0x2A-31     JGE     [imm/var]   [imm/var]   [imm/var]
[ ] 0x32-39     JG      [imm/var]   [imm/var]   [imm/var]
[ ] 0x3A-41     JLE     [imm/var]   [imm/var]   [imm/var]
[ ] 0x42-49     JL      [imm/var]   [imm/var]   [imm/var]
[ ] 0x4A-4B     MOV     [imm/var]   [var]
[ ] 0x4C-4F     AND     [imm/var]   [imm/var]   [var]
[ ] 0x50-53     OR      [imm/var]   [imm/var]   [var]
[ ] 0x54-57     XOR     [imm/var]   [imm/var]   [var]
[ ] 0x58-59     NOT     [imm/var]   [var]
[ ] 0x5A-5D     LSH     [imm/var]   [imm/var]   [var]
[ ] 0x5E-61     RSH     [imm/var]   [imm/var]   [var]
[ ] 0x62-63     VAR     [type/var]  [name]
[ ] 0x64-65     RET     [imm/var]
[ ] 0x66-67     DEREF   [ptr]       [var]
[ ] 0x68-69     REF     [var]       [ptr var]
[ ] 0x6A-6B     INST    [name/var]  [var]
```

## TYPES

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

## IMMEDIATE VALUES
Immediate values are values that are stored within the bytecode instructions themselves. These values are used for all non-pointer data types.
An example of an instruction that uses immediate values is the creation of a variable with type i32 and value 5 is below.
```
62 03 01 61
4A 03 00 00 00 05 01 61
```

## BYTECODE STRINGS
Bytecode strings are how function names/variable names/etc. are represented in the bytecode.
Their format is as follows
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

## DATA SECTION
This is a section of the bytecode where all constants (i.e. strings, arrays) are stored for use in the program.
Immediate values (numbers) are not stored in this section, and instead are stored in the instructions themselves.
This section is placed at the end of the file.
The format is as follows
```
FC
(name) (data type) (length bytes) (data length) (data)
(name) (data type) (length bytes) (data length) (data)
...
```
The amount of bytes in the data length is specified by the bytes beforehand.

## DATA CONSTANTS
Data constants are used for defining data inside of pointers/arrays in the bytecode.
Data constants are treated as variables in the global scope.
The format is as follows
```
(name) (data type) (length bytes) (data length) (data)
```

## SCOPES
Scopes can be defined anywhere in the bytecode. Scopes are defined using the reserved bytes FE (scope start) and FD (scope end) as listed above.
TODO: add more info on scopes

## FUNCTIONS
The format for functions is as follows
```
FF (return type) (function name) (args) FE
    (code)
FD
```
Arguments are defined as follows
```
(type) (name) (type) (name) ...
```
An example function would look like this:
```
FF 01 04 6D 61 69 6E 0C 0C 05 04 61 72 67 73 FE
    (code)
FD
```
In pseudocode this would be
```c++
byte main(char** args) {

}
```

## STRUCTS
Structs are custom data structures that contain variables.
Their format is as follows
```
FB (name) FE
(var)
(var)
...
FD
```
An example struct would look like this
```
FB 03 46 6F 6F FE ; struct Foo {
03 01 61 ; i32 a
0A 01 62 ; f32 b
0C 05 03 74 78 74 ; char* txt
FD ; }
```
In pseudocode this would be
```rust
struct Foo {
    i32 a
    f32 b
    char* txt
}
```
The values within structs are accessed through the normal instructions used for variables, with the format of
```
(struct instance name).(field)
```

## ERRORS
Error handling is currently undefined in Rainbow.

## OPTIMIZATIONS
Optimizations are currently not implemented for Rainbow.