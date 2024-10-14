# RAINBOW BYTECODE SPECIFICATION

## RESERVED BYTES
```
0xFF - function start
0xFE - scope start
0xFD - scope end
0xFC - data section start
0xFB - struct start
0xFA - file import
0xF9 - extern function
0xF8 - extern args end
0xF7 - conditional parse
0xF6 - module start
0xF5 - unused
0xF4 - unused
0xF3 - unused
0xF2 - unused
0xF1 - unused
0xF0 - unused
```

## INSTRUCTIONS

Each instruction consists of an opcode followed by its arguments.

```
[x] 0x00        NOP
Does nothing

[x] 0x01-02     PUSH    [imm/var]
Pushes value A onto the stack

[x] 0x03        POP     [var]
Pops a value off of the stack and stores it in variable A
With a stack underflow, the program will crash

[x] 0x04-0x05   PEEK    [imm/var]   [var]
Copies value from the stack at index A and stores it in variable B

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

[x] 0x50-53     AND     [imm/var]   [imm/var]   [var]
Perform bitwise AND on A and B and store in variable C

[x] 0x54-57     OR      [imm/var]   [imm/var]   [var]
Perform bitwise OR on A and B and store in variable C

[x] 0x58-5B     XOR     [imm/var]   [imm/var]   [var]
Perform bitwise XOR on A and B and store in variable C

[x] 0x5C-5D     NOT     [imm/var]   [var]
Perform bitwise NOT on A and B and store in variable C

[x] 0x5E-61     LSH     [imm/var]   [imm/var]   [var]
Left shift A by B bits and store in variable C

[x] 0x62-65     RSH     [imm/var]   [imm/var]   [var]
Right shift A by B bits and store in variable C

[x] 0x66-69     VAR     [type/var]  [name/var]
Create a variable with type A and name B

[x] 0x6A-6C     RET     {imm/var}
Return value A from a function
(functions with void type do not need to include arguments)

[x] 0x6D-6E     DEREF   [imm/ptr]   [var]
Dereference pointer A and store in variable B
(note: deref clones the value that you are dereferencing)

[x] 0x6F-70     REF     [imm/var]   [ptr var]
Create a reference to value A and store in variable B

[x] 0x71-72     INST    [name/var]  [var]
Instantiate a struct named A and store in varaible B
(Struct will be filled with 0s or empty values)

[x] 0x73-76     MOD     [imm/var]   [imm/var]   [var]
Perform modulus on A and B and store in variable C

[x] 0x77-7A     PMOV    [imm/var]   [ptr var]   [imm/var]
Moves the value A into where value B references, with the offset C

[x] 0x7B-7E     ALLOC   [type/var]  [imm/var]   [ptr var]
Allocates a pointer with type A, size B, and puts the address in variable C

[x] 0x7F-83     FREE    [imm/ptr]   {imm/var}
Frees pointer A with size B
Size only needs to be provided when given an immediate address, but still can be provided given a pointer variable
If size is not provided with the given pointer variable the pointer will be deleted, if a size is provided the pointer will remain.

[ ] 0x84-8B     CALLC   [imm/var]   [type/var]  [imm/var]
Calls the function in memory at address A, return type B, and argument count C.

[x] 0x8C-93     CMP     [imm/var]   [imm/var]   [imm/var]   [var]
Compares B to C with condition A, and stores 1 or 0 in D depending on the result.

0x00: ==
0x01: !=
0x02: >=
0x03: >
0x04: <=
0x05: <
```

0xXX-0xYY - instruction opcode range
Counted up by argument type, futher explained below.

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


## OPCODE RANGES AND ARGUMENT COMBINATIONS

For a given range of opcodes, the instructions differ by changing the argument types in a set order. The argument types change systematically from left to right.

### Example:
Opcode range `0x22-29` (the JE instruction) corresponds to the following instructions:

- `0x22` [imm] [imm] [imm]
- `0x23` [var] [imm] [imm]
- `0x24` [imm] [var] [imm]
- `0x25` [var] [var] [imm]
- `0x26` [imm] [imm] [var]
- `0x27` [var] [imm] [var]
- `0x28` [imm] [var] [var]
- `0x29` [var] [var] [var]

In this example, `imm` and `var` are just two of the possible argument types, with other combinations being possible. The types increment based on a fixed order, with each new opcode reflecting a new combination.

For opcodes with optional arguments, the counting is very similar, however instructions with the optional argument are put first.

### Example:
Opcode range `0x7F-83` (the FREE instruction) corresponds to the following instructions:

- `0x7F` [var]
- `0x80` [imm] [imm]
- `0x81` [var] [imm]
- `0x82` [imm] [var]
- `0x83` [var] [var]

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
66 03 01 61
4A 03 00 00 00 05 01 61
```

## BYTECODE STRINGS
Bytecode strings are how function names/variable names/etc. are represented in the bytecode.
Their format is as follows
```
(length - 1 byte) (characters)
```
An example string would look like this:
```
0D 48 65 6C 6C 6F 2C 20 57 6F 72 6C 64 21
```

The first byte `0D` represents the length of the string, in this case 13.
The rest of the bytes are the text in byte form.

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
(name) (data type) (length type) (length) (data)
(name) (data type) (length type) (length) (data)
...
```
The amount of bytes in the data length is specified by the bytes beforehand.

## DATA CONSTANTS
Data constants are used for defining data inside of pointers/arrays in the bytecode.
Data constants are treated as variables in the global scope.
The format is as follows
```
(name) (data type) (bytes needed for length) (length) (data)
```

## SCOPES
Scopes can be defined anywhere in the bytecode, except for inside structs. Scopes are defined using the reserved bytes FE (scope start) and FD (scope end) as listed above.
Scopes can be defined recursively, with scopes inside scopes being possible.

## FUNCTIONS
Functions can be defined anywhere in the bytecode, except for inside structs. You can have functions in functions if you want.

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
i8 main(char** args) {

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

The variables are defined as `(type) (name)`, where type is a type from the list of types, and the name is a bytecode string.

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

## IMPORTS
The format of imports is
`FA (imported file as bytecode string)`

## CONDITIONAL PARSING
Conditional parsing allows you to toggle any part of your code based off of constant variables. These variables are provided by either the runtime or the user.
The format is
```
F7 (type) (variable name) (condition) (variable name)
    FE
        (body)
    FD
...repeat...
```

the types are
```
00: if
01: else if
02: else
03: end
```

the conditions are
```
00: ==
01: !=
02: >=
03: >
04: <=
05: <
```

```c#
.if PLATFORM == PLATFORM_WIN32
    {code}
.elseif PLATFORM == PLATFORM_LINUX
    {code}
.end
```

becomes

```
F7 00 08 50 4C 41 54 46 4F 52 4D 00 0F 50 4C 41 54 46 4F 52 4D 5F 57 49 4E 44 4F 57 53 
    FE
        ...
    FD
F7 01 08 50 4C 41 54 46 4F 52 4D 00 0D 50 4C 41 54 46 4F 52 4D 5F 4C 49 4E 55 58 
    FE
        ...
    FD
F7 03
```

To pass in constant variables use `--const FOO=123` or `-c FOO=456`

## EXTERNS
Externs are ways of importing functions from compiled code into your Rainbow code.

The format is
```
F9 (return type) (name) (arg types) F8 (extern file)
```

```
extern u64 GetStdHandle(i32) @"Kernel32.dll"
```

becomes

```
F9 08 0C 47 65 74 53 74 64 48 61 6E 64 6C 65 03 F8 0C 4B 65 72 6E 65 6C 33 32 2E 64 6C 6C
```

## MODULES
Modules are ways of grouping and organizing code.

The format is
```
F6 (module name) FE
    ... code
FD
```

```
.module io {
    ...
}
```

becomes

```
F6 02 69 6F FE
    ...
FD
```

## ERRORS
Error handling is currently undefined in Rainbow.

## OPTIMIZATIONS
Optimizations are currently not implemented for Rainbow.