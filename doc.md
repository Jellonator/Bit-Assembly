# Bit Assembly Master Document #
Bit assembly is an assembly-inspired language that uses bits rather than bytes.
Not that it's not actually assembly.

## Basics ##
Data in bit asm is stored(of course) as a sequence of bits.

Quick 'Hello, World!' example:
```
push 1024             ; Push 1024 Kb onto the stack

pop 1024              ; Pop the rest of the data
```

### Literals ###
literals are denoted without any special symbols, e.g. `1234`, `0xff` or `'foo'`

### Pointers ###
Using a percent symbol and a numeric literal, such as `%1234`, can be used to refer to specific memory locations. in this case it refers to memory address 1234.

### Dereferencing ###
The value of a pointer can be retrieved by using the dereference symbol, e.g. `*1234`. If the value retrieved is also a pointer, that too can be dereferenced like so: `**1234`.

### Variable length bytes ###
All memory addresses are one bit, but bits can be chained together to form a byte. For example, `%12[8]` is a pointer to memory location 12, along with the next 7 bits after it(length 8).

### Comments ###
Comments are anything after a semicolon:
```
;this is a comment
mov %0, 0 ;this is also a comment
```

### Strings ###
```

```

### Memory Length Omission ###
The length of certain types can be omitted, such as static strings and characters, which both have definite sizes at time of compilation. A character will have a length of 8 bits, and a string will have a length of 8 bits for every character it contains.

Char example:
```
mov %0[], 'a'  ;set first 8 bits to value of 'a'
```

String example:
```
mov %0[], "Foobar" ;set first 8*6(48) bits to 'foobar'
```

### Signed and Unsigned numbers ###
By default, all numbers are signed. In order to get a number as unsigned, the array notation should use the negative symbol as shown here: `&0[-8]`.

## Memory ##
### The Stack ###
By default, no bits are allocated on the stack. In order to get memory for use, they must be allocated to the stack. More memory can be allocated with the `push` and `pop` instructions.

Push takes the format `push [size], {value}`, where 'size' is how many bits need to be pushed onto the stack. The optional argument 'value' is the value of the bits to be pushed, and 'size' must be divisible by the size of 'value'. Otherwise, pushed bits will be zeroed.

Push takes the format `pop [size], {value}`, where 'size' is how many bits are popped from the stack. The optional argument 'value' is a pointer with a size of 'size' that will take the value of the popped value.

A String can be pushed and popped from the stack like so:
```
push #foo, *foo
pop #foo
.const
	foo: "Hello, World!"
```

The following examples assume that the necessary bits have already been allocated.

## Standard Input and Output ##
### stdout ###

## Instructions ##
### MOV ###
the `mov` instruction can copy values into memory with the format `mov destination, source` where destination is a pointer and source is a value(literal or dereferenced pointer).
Examples:
```
mov %0, 1       ;set address 0
mov %1[8], 200
```

You can not `mov` values of differing sizes, The following will not work:
```
mov %6[3], 128 ;128 will not fit into 3 bits(max 8)

mov %0[4], 2     ;assign first four bits to two
mov %4[2], *0[4] ;Despite two being able to fit into two bytes, you can not fit four bits into two bits.

mov %0[2], 2     ;assign first two bits to two
mov %2[4], *0[2] ;This also won't work, even though two bits could fit into four bits, they are not of the same size.
```

### NOT operator ###
The NOT operator has the format `not destination, source`, where destination is a pointer and source is a value(literal or dereferenced pointer). NOT will take the value of 'source,' invert each bit, and assign 'destination' to the result.
Example:
```
mov %0,  0 ;set memory location 0 to 0
not %1, *0 ;set memory location 1 to the inverse of the value in memory location 0 (not 0 = 1)

mov %0[8], 200  ;11001000
not %8[8], *0[8];00110111, or 55
```

Not can also take the form `not value`, where value is a pointer, in order to flip a value in-place.

### Binary operators ###
Bits can be operated on using OR, XOR, AND, etc.
All of these instructions take the format `and destination, op1, op2` where op1 and op2 are values of the same length, and destination is a pointer with either the same length as op1 and op2 or a length of 1.

### ADD and SUB ###
Bits can be operated on with the arithmetic operators such as ADD and SUB.
ADD and SUB both take the format `add destination, op1, op2` where destination is a pointer, op1 and op2 are values, and destination, op1, and op2 all have the same size and are all bytes.

The ADD operator will take the values of op1 and op2, and add them together:
```
add %0[8], 40, 40 ;assign memory location 0 to 40 + 40, which is 80
```

The SUB operator will take the value of op1, subtract op2, and assign it to destination:
```
sub %0[8], 100, 33 ;assign memory location 0 to 100 + 30, which is 70
```

### MUL ###
The MUL operator is used to multiply two values and store it into another. It takes the format `mul destination, op1, op2` where destination is a pointer, op1 and op2 are values, and destination, op1, and op2 all have the same size and are all bytes.
Example:
```
mul %0[8], 8, 12 ;assign memory location 0 to 8 * 12, which is 96.
```

### DIV ###
The DIV operator divides one number by another and assigns the result to destination. It takes the format `div destination, op1, op2` where destination is a pointer, op1 and op2 are values, and destination, op1, and op2 all have the same size and are all bytes.
Example:
```
div %0[8], 200, 5 ;assign memory location 0 to 200/5, which is 40.
```
