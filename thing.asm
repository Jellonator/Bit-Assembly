push 32
mov [0:8], 10
mov [8:8], 5
mov [16:8], 0
mov [24:8], 0

.mloop
add [16:8], [16:8], [8:8]
sub [0:8], [0:8], 1
jne mloop, [0:8], 0
