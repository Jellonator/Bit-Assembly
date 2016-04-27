# Bit Assembly Master Document #
Bit assembly is an assembly-inspired language that uses bits rather than bytes.
Not that it's not actually assembly.

## Basics ##
Data in bit asm is stored(of course) as a sequence of bits.

Quick 'Hello, World!' example:
```
push 256
mov [0:1024], "Hello, World!"
ext print, "hello, World!"
```

### Literals ###
literals are denoted without any special symbols, e.g. `1234`, `0b1100` or `"foobar"`
`1234` is a base-10 numeric literal,
`0b1100` is a base-2 binary literal,
`"foobar"` is a string value, which is converted into binary.

### Pointers ###
A pointer is denoted with square brackets, e.g. `[2]` refers to bit 2 and `[8:16]` refers to bits 8 through 23(size of 16 bits).

### Comments ###
Comments are anything after a semicolon:
```
;this is a comment
mov [0], 0 ;this is also a comment
```

## Memory ##
### The Stack ###
By default, no bits are allocated on the stack. In order to get memory for use, they must be allocated to the stack. More memory can be allocated with the `push` and `pop` instructions.

Push takes the format `push [size]`, where 'size' is how many bits need to be pushed onto the stack.

Push takes the format `pop [size]`, where 'size' is how many bits are popped from the stack.

The following examples assume that the necessary bits have already been allocated.

## External calls ##
The `ext` instruction, also known as 'external call', is used to interact with outside interfaces. Currently, it contains only string input and output functions, but it is easily extensible.

### print ###
Using `ext print, value` can be used to print out a string to the standard output. This function will cause a crash if the provided string is not valid utf-8. Strings may be null terminated.

Likewise, `ext printnum, value` is used to print out numeric values to the standard output.

### input ###
`ext input, pointer` will take in a string input, and assign it to 'pointer'.

`ext inputnum, pointer` will take in a string input, convert it into a number, and assign it to 'pointer'.

`ext valid, pointer` can be used to check if the input was valid or not. 'pointer' only needs to be one bit.

## Instructions ##
Instructions tell the compiler what to do. They are very similar to instructions in other assembly languages, but since Bit assembly does not use registers, it is a little more verbose. All instructions take the form `instruction {arg1}, {arg2}...`, where arguments are separated by commas.

### MOV ###
the `mov` instruction can copy values into memory with the format `mov destination, source` where destination is a pointer and source is a any value.
Examples:
```
mov [0], 1        ; set address 0 to 1
mov [8:8], 200    ; set addresses 8:8 to 200
mov [16:8], [8:8] ; copy [8:8] to [16:8]
```

You can not `mov` a larger value into a smaller balue;
```
mov [0:3], 128   ;128 will not fit into 3 bits(minimum 8 bits)

mov [0:4], 2     ;assign first four bits to two
mov [0:2], [0:4] ;Despite two being able to fit into two bytes, you can not fit four bits into two bits.

mov [0:2], 2     ;assign first two bits to two
mov [0:4], [0:2] ;This does work, since 2 bits can be coerced into four.
```

### NOT operator ###
The NOT operator has the format `not destination, source`, where destination is a pointer and source is any value. NOT will take the value of 'source,' invert each bit, and assign 'destination' to the result.
Example:
```
mov [0],  0  ;set memory location 0 to 0
not [1], [0] ;set memory location 1 to the inverse of the value in memory location 0 (not 0 = 1)

mov [0:8], 200   ; 11001000
not [8:8], [0:8] ; 00110111, or 55
```

### Binary operators ###
Bits can be operated on using OR, XOR, AND, etc.
All of these instructions take the format `and destination, op1, op2` where op1 and op2 are values of the same length, and destination is a pointer with either the same length as op1 and op2 or a length of 1.

### ADD and SUB ###
Bits can be operated on with the arithmetic operators such as ADD and SUB.
ADD and SUB both take the format `add destination, op1, op2` where destination is a pointer, op1 and op2 are values, and destination, op1, and op2 all have the same size and are all bytes.

The ADD operator will take the values of op1 and op2, and add them together:
```
add [0:8], 40, 40 ;assign memory location 0 to 40 + 40, which is 80
```

The SUB operator will take the value of op1, subtract op2, and assign it to destination:
```
sub [0:8], 100, 33 ;assign memory location 0 to 100 + 30, which is 70
```

### MUL ###
The MUL operator is used to multiply two values and store it into another. It takes the format `mul destination, op1, op2` where destination is a pointer, op1 and op2 are values, and destination, op1, and op2 all have the same size and are all bytes.
Example:
```
mul [0:8], 8, 12 ;assign memory location 0 to 8 * 12, which is 96.
```

### DIV ###
The DIV operator divides one number by another and assigns the result to destination. It takes the format `div destination, op1, op2` where destination is a pointer, op1 and op2 are values, and destination, op1, and op2 all have the same size and are all bytes.
Example:
```
div [0:8], 200, 5 ;assign memory location 0 to 200/5, which is 40.
```
