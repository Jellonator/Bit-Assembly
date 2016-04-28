# Bit Assembly Master Document #
Bit assembly is an assembly-inspired language that uses bits rather than bytes.
Not that it's not actually assembly.

Note that this isn't a serious project and that it is mostly a joke about mixing a hard to use group of languages with a data type that is awkward to manage.

Also note that programs written with Bit Assembly are going to be pretty slow, due to the nature of bignums and lazy programmer inefficiencies.

In this guide, necessary arguments are shown in `[brackets]`, whereas unnecessary arguments are shown in `{curly brackets}`.

## Basics ##
Data in bit asm is stored(of course) as a sequence of bits.

Quick 'Hello, World!' example:
```asm
push 256
mov [0:256], "Hello, World!"
ext print, "hello, World!"
```

### Literals ###
literals are denoted without any special symbols, e.g. `1234`, a base-10 numeric literal, `0b1100`, a base-2 binary literal, or `"foobar"`, a string literal which is converted into a binary literal.

### Pointers ###
A pointer is denoted with square brackets, e.g. `[2]` refers to bit 2 and `[8:16]` refers to bits 8 through 23(size of 16 bits).

### Comments ###
Comments are anything after a semicolon:
```asm
;this is a comment
mov [0], 0 ;this is also a comment
```

## Memory ##
### The Stack ###
By default, no bits are allocated on the stack. In order to get memory for use, they must be allocated to the stack. Memory can be allocated or deallocated with the `push` and `pop` instructions, respectively.

Push takes the format `push [size]`, where 'size' is how many bits need to be pushed onto the stack.

Push takes the format `pop [size]`, where 'size' is how many bits are popped from the stack.

The following examples assume that the necessary bits have already been allocated.

## External calls ##
The `ext` instruction, also known as 'external call', is used to interact with outside interfaces. Currently, it contains only string input and output functions, but it is easily extensible.

### print ###
Using `ext print, [value]` can be used to print out a string to the standard output. This function will cause a crash if the provided string is not valid utf-8. Strings may be null terminated.

Likewise, `ext printnum, [value]` is used to print out numeric values to the standard output.

### input ###
All input must first be attained via `ext prompt`, which will prompt the user for an input, and store it.

`ext input, [pointer]` will take in the string input from the prompt, and store it into the pointer.

`ext inputnum, [pointer]` will take the input, convert it into a number, and store it into 'pointer'.

`ext valid, [pointer]` can be used to check if the input was valid or not. 'pointer' only needs to be one bit, but it can be any length anyways.

`ext inputlen, [pointer]` is used to get the length of the input, and store it into the pointer.

## Instructions ##
Instructions tell the compiler what to do. They are very similar to instructions in other assembly languages, but since Bit assembly does not use registers, it is a little more verbose. All instructions take the form `instruction [arg1], [arg2]...`, where arguments are separated by commas.

### MOV ###
the `mov` instruction can copy values into memory with the format `mov [destination], [source]` where destination is a pointer and source is a any value.
Examples:
```asm
mov [0], 1        ; set address 0 to 1
mov [8:8], 200    ; set addresses 8:8 to 200
mov [16:8], [8:8] ; copy [8:8] to [16:8]
```

You can not `mov` a larger value into a smaller value;
```asm
mov [0:3], 128   ;128 will not fit into 3 bits(minimum 8 bits)

mov [0:4], 2     ;assign first four bits to two
mov [0:2], [0:4] ;Despite two being able to fit into two bytes, you can not fit four bits into two bits.

mov [0:2], 2     ;assign first two bits to two
mov [0:4], [0:2] ;This does work, since 2 bits can be coerced into four.
```

### NOT operator ###
The NOT operator has the format `not [destination], [source]`, where destination is a pointer and source is any value. NOT will take the value of 'source,' invert each bit, and assign 'destination' to the result.
Example:
```asm
mov [0],  0  ;set memory location 0 to 0
not [1], [0] ;set memory location 1 to the inverse of the value in memory location 0 (not 0 = 1)

mov [0:8], 200   ; 11001000
not [8:8], [0:8] ; 00110111, or 55
```

### Binary operators ###
Bits can be operated on using OR, XOR, AND, etc.
All of these instructions take the format `and [destination], [op1], [op2]` where op1 and op2 are values of the same length, and destination is a pointer with either the same length as op1 and op2 or a length of 1.

### ADD and SUB ###
Bits can be operated on with the arithmetic operators such as ADD and SUB.
ADD and SUB both take the format `add [destination], [op1], [op2]` where destination is a pointer, op1 and op2 are values, and destination, op1, and op2 all have the same size and are all bytes.

The ADD operator will take the values of op1 and op2, and add them together:
```asm
add [0:8], 40, 40 ;assign memory location 0 to 40 + 40, which is 80
```

The SUB operator will take the value of op1, subtract op2, and assign it to destination:
```asm
sub [0:8], 100, 33 ;assign memory location 0 to 100 - 33, which is 67
```

### MUL ###
The MUL operator is used to multiply two values and store it into another. It takes the format `mul [destination], [op1], [op2]` where destination is a pointer, op1 and op2 are values, and destination, op1, and op2 all have the same size and are all bytes.
Example:
```asm
mul [0:8], 8, 12 ;assign memory location 0 to 8 * 12, which is 96.
```

### DIV ###
The DIV operator divides one number by another and assigns the result to destination. It takes the format `div [destination], [op1], [op2]` where destination is a pointer, op1 and op2 are values, and destination, op1, and op2 all have the same size and are all bytes.
Example:
```asm
div [0:8], 200, 5 ;assign memory location 0 to 200/5, which is 40.
```

## Control Flow ##
The control flow can be controlled using jumps and calls, much like other assembly languages.

### Jumps ###
A basic jump can be done with the `jmp` instruction. It takes the format `jmp [label]`, where label is a valid label name.

Comparative jumps can be achieved with other instructions. Such instructions include 'je op1, op2, label' which jumps to 'label' if op1 is equal to op2.

Other Jumps include the following:

 * jl: jump less than
 * jle: jump less than or equal to
 * jg: jump greater than
 * jge: jump greater than or equal to
 * jne: jump not equal to

### Calls ###
Bit Assembly also includes a call stack. The `call [label]` instruction pushes is similar to the `jmp` instruction, but it also adds the next instruction to the call stack. When the `ret` instruction is used, it jumps to the instruction after the last call, and pops the end off of the call stack.

The following example illustrates this:
```asm
jmp start

.callme
	ext print, "This is printed"
	ret
	ext print, "This is never printed"

.start
	call callme
	ext print, "This is printed too!"
```

## Macros ##
Bit assembly also includes a few simple macros to make using it a little easier

### !define ###
The `!define [name] {value}` macro will define a metavariable, which will replace all instances of its name with its value. Since Bit Assembly doesn't have registers, the define macro allows the programmer to define their own.

```asm
push 4
!define register1 [0]
!define register2 [1]
!define register3 [2]
!define register4 [3]
mov register1, 0 ; set register 1 to 0
mov register2, 1 ; set register 2 to 0
and register3, register1, register2 ; and registers 2 and 1 together, store it into register 3.
or  register4, register1, register2 ; or registers 2 and 1 together, store it into register 4.
```

### !include ###
The `!include [filename]` macro will take every line of another file and load it into the assembler at the position of the macro. This is useful for splitting up code into multiple files.
