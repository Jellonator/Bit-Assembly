push 256;first few bits are register
mov [0:256], 0
jmp start;

.start
ext printnum, 12
