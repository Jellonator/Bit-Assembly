push 256;first few bits are register
mov [0:256], 0
jmp start;

.start
mov [  0:64], 10
mov [ 64:64], 10
mov [128:64],  0
mov [-1:1], 0

.first
ext print, "Enter first number: "
ext inputnum, [0:64]
ext valid, [-1:1]
je [-1:1], 0, first

.second
ext print, "Enter second number: "
ext inputnum, [64:64]
ext valid, [-1:1]
je [-1:1], 0, second

ext print, "Multiplying "
ext printnum, [0:64]
ext print, " by "
ext printnum, [64:64]

mul [128:64], [0:64], [64:64]

ext print, "\nResult: "
ext printnum, [128:64]
