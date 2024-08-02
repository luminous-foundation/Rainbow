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
[x] 0x00        NOP
Does nothing

[x] 0x01-02     PUSH    [imm/var]
Pushes value A onto the stack

[x] 0x03        POP     [var]
Pops a value off of the stack and stores it in variable A
With a stack underflow, the program will crash

[x] 0x04-05     LDARG   [imm/var]
Loads argument A to be used in a function
(This is currently functionally identical to PUSH)

[x] 0x06-07     CALL    [func/var]
Calls function A

[x] 0x08-0B     ADD     [imm/var]   [imm/var]   [var]
Add A to B and store in variable C

[x] 0x0C-0F     SUB     [imm/var]   [imm/var]   [var]
Subtract B from A and store in variable C

[x] 0x10-13     MUL     [imm/var]   [imm/var]   [var]
Multiply A by B and store in variable C

[x] 0x14-17     DIV     [imm/var]   [imm/var]   [var]
Divide A by B and store in variable C

[x] 0x18-19     JMP     [imm/var]
Jump to location A within the current scope

[x] 0x1A-21     JNE     [imm/var]   [imm/var]   [imm/var]
Jump to location C within the current scope if the given values are not equal

[x] 0x22-29     JE      [imm/var]   [imm/var]   [imm/var]
Jump to location C within the current scope if the given values are equal

[x] 0x2A-31     JGE     [imm/var]   [imm/var]   [imm/var]
Jump to location C within the current scope if value A is greater than or equal to B

[x] 0x32-39     JG      [imm/var]   [imm/var]   [imm/var]
Jump to location C within the current scope if value A is greater than to B

[x] 0x3A-41     JLE     [imm/var]   [imm/var]   [imm/var]
Jump to location C within the current scope if value A is less than or equal to B

[x] 0x42-49     JL      [imm/var]   [imm/var]   [imm/var]
Jump to location C within the current scope if value A is less than to B

[x] 0x4A-4F     MOV     [imm/*var]   [*var]
Move value A into variable B

[ ] 0x50-53     AND     [imm/var]   [imm/var]   [var]
Perform bitwise AND on A and B and store in variable C

[ ] 0x54-57     OR      [imm/var]   [imm/var]   [var]
Perform bitwise OR on A and B and store in variable C

[ ] 0x58-5B     XOR     [imm/var]   [imm/var]   [var]
Perform bitwise XOR on A and B and store in variable C

[ ] 0x5C-5D     NOT     [imm/var]   [var]
Perform bitwise NOT on A and B and store in variable C

[ ] 0x5E-61     LSH     [imm/var]   [imm/var]   [var]
Left shift A by B bits and store in variable C

[ ] 0x62-65     RSH     [imm/var]   [imm/var]   [var]
Right shift A by B bits and store in variable C

[x] 0x66-69     VAR     [type/var]  [name/var]
Create a variable with type A and name B

[x] 0x6A-6C     RET     {imm/var}
Return value A from a function
(functions with void type do not need to include arguments)

[x] 0x6D-6E     DEREF   [imm/ptr]   [var]
Dereference pointer A and store in variable B
(note: deref clones the value that you are dereferencing)

[x] 0x6F        REF     [var]       [ptr var]
Create a reference to variable A and store in variable B

[ ] 0x70-71     INST    [name/var]  [var]
Instantiate a struct named A and store in varaible B
(Struct will be filled with 0s or empty values)

[x] 0x72-75     MOD     [imm/var]   [imm/var]   [var]
Perform modulus on A and B and store in variable C

[ ] 0x76-79     PMOV    [imm/var]   [ptr var]   [imm/var]
Moves the value A into where value B references, with the offset C

[ ] 0x7A-7D     ALLOC   [type/var]  [imm/var]   [ptr var]
Allocates a pointer with type A, size B, and puts the address in variable C

[ ] 0x7E-82     FREE    [imm/ptr]   {imm/var}
Frees pointer A with size B
(Size only needs to be provided when given immediate address, but still can be provided given pointer variable)
```

0xXX-0xYY - instruction opcode range
Counted up by argument type. (TODO: qualify this better)

[...]     - argument
These arguments are required.

{...}     - optional argument
These arguments are not required.

imm       - immediate value
This is an immediate value as specified below.

var       - variable name
This is a variable name, represented as a bytecode string as specified below.

ptr var   - variable name, where the variable holds a pointer
This is also a variable name represented as a bytecode string, but the specified variable must have the pointer type.

*var      - specifies a dynamically named variable is supported
The argument you pass in is either a statically named var that contains the value you are trying to move,
or a variable that contains the name of the variable you want to move.


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
Their format is as follows
```
(type) (value)
```
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