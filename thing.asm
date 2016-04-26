push 256;first few bits are register
mov [0:256], 0
jmp start;

!define rax [  0:64]
!define rbx [ 64:64]
!define rcx [128:64]
!define rdx [192:64]

.start
	mov rax, 10
	mov rbx, 10
	mov rcx,  0
	mov rdx,  0

.first
	ext print, "Enter first number: "
	ext inputnum, rax
	ext valid, rdx
	je rdx, 0, first

.second
	ext print, "Enter second number: "
	ext inputnum, rbx
	ext valid, rdx
	je rdx, 0, second

	ext print, "Multiplying "
	ext printnum, rax
	ext print, "\nby          "
	ext printnum, rbx

	mul rcx, rax, rbx

	ext print, "\nResult:     "
	ext printnum, rcx
