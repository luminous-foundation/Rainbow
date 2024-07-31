# The Rainbow Bytecode Runtime

## What is it?
The Rainbow Bytecode Runtime (referred to as just Rainbow, the Rainbow Bytecode, the Rainbow Runtime, or some other combination that involves Rainbow) is a bytecode runtime built with the philosiphy of giving the developer ultimate freedom, while still remaining as fast as possible.

## How do I use it?
If you just want to run a Rainbow file (.rbb), currently there is no way to do it without editing `main.rs` however a method to do so will be provided soon.

### If you want to develop programs for Rainbow however
The current recommended method for programming for Rainbow is to use [RASM](https://github.com/luminous-foundation/rasm).

However, if you would like to use Rainbow for your language (or so desire to program in raw bytecode) you can refer to [the spec](spec.md)
Here is also a quick example of a program that adds two numbers, and stores the result in a variable.

```
66 05 01 61 
08 01 01 01 02 01 61
```

`66 05 01 61`
This line creates a variable named `a`, with type u8.

The first byte is the opcode, `66` denoting the VAR instruction with the arguments of `type` and `name`.

The second byte is the type.

The last two bytes are the name of the variable (according to the bytecode string format specified in [the spec](spec.md))


`08 01 01 01 02 01 61`

The first byte is again the opcode, this time `08` denoting the ADD instruction with the arguments of `imm`, `imm`, and `var`.

`imm` specifying an immediate value as according to the format specified in [the spec](spec.md).

`var` is again a bytecode string, this time containing the name of the variable you are trying to store the output of the addition into.

The next two bytes are an immediate value of type u8 with a value of 1.

The next two bytes are an immediate value of type u8 with a value of 2.

The last two bytes are a bytecode string holding the name of the variable `a`.

## Am I allowed to make a programming language/project/whatever using Rainbow?
Yes, and it is encouraged.

The more things that can run on Rainbow the more things can talk to each other through Rainbow.

## What is the future plans for Rainbow?
Currently the below list of features is planned for Rainbow.

- [ ] Self-hosted

- [ ] Language interop (FFI, other bytecodes, etc.)

- [ ] JIT Compilation

- [ ] AOT Compilation

## How can I contribute?
Currently the only way to contribute to Rainbow is through making issues and pull requests.

There is no contribution guidelines *yet*, but hopefully soon there will be.