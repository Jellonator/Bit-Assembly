push 256;first few bits are register
mov [0:256], 0
jmp start;

!define eax [ 0:32]
!define ebx [32:32]
!define ecx [64:32]
!define edx [96:32]

.divide
	push 96
	mov [-32:32], eax ; dividend
	mov [-64:32], ebx ; divisor
	mov [-96:32],   0 ; quotient
	mov     edx , eax ; original
	.divide_loop
		jl  [-32:32], [-64:32], divide_end ; end loop if can not be subtracted
		sub [-32:32], [-32:32], [-64:32]   ; subtract divisor from dividend
		add [-96:32], [-96:32],       1    ; add one to result
		jmp divide_loop
	.divide_end
	mov ecx, [-64:32] ; divisor
	mov ebx, [-32:32] ; remainder
	mov eax, [-96:32] ; result
	pop 96
	ret

.start
	push 64
	mov [-32:32], 999
	mov [-64:32],  10
	mov eax, [-32:32] ; put 200 into register 0
	mov ebx, [-64:32] ; put 10 into register 1
	call divide
	pop 64
