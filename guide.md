# RAINBOW BYTECODE GUIDE

## very work in progress

Here is how you add two numbers in Rainbow.

This is how you do it in RASM.
```
VAR i16 result

ADD 1 2 result
```

This is the bytecode representation of the above code.
```
66 02 06 72 65 73 75 6C 74
08 02 00 01 02 00 02 06 72 65 73 75 6C 74
```

Let's break this down.

`66` - This is specifying that a variable is going to be created.

`02` - This is the type of the variable, in this case i16.

`06` - This is the length of the variable name.

`72 65 73 75 6C 74` - This is the variable name.

Now the next line.

`08` - This is specifying that we are about to add two numbers.

`02` - This is the type of the first number, again in this case i16.

`00 01` - This is the value of the first number.

`02` - This is the type of the second number.

`00 02` - This is the value of the second number.

`06` - This is the length of the name of the variable that the result will go into.

`72 65 73 75 6C 74` - This is the variable name.

Let's have another example, this time adding three numbers.

Again, here is the RASM version
```
VAR i16 result

ADD 1 2 result
ADD 3 result result
```

And here is that in bytecode form.
```
66 02 06 72 65 73 75 6C 74
08 02 00 01 02 00 02 06 72 65 73 75 6C 74
0A 02 00 03 06 72 65 73 75 6C 74 06 72 65 73 75 6C 74
```

As you can see, the first two lines are the same, but the third line is different.

So let's break down that third line.

`0A` - This is specifying that we are about to add two numbers, except that the second number is a variable.

`02` - Once again, this is the type of the first number

`00 03` - This is the value of the first number

`06` - This is the length of the name of the variable that is the second number

`72 65 73 75 6C 74` - This is the name of the variable