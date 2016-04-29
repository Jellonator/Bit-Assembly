push 512

jmp start

!define top <[-0]
!define rax [0:8]
!define rbx [8:8]
!define rcx [16:8]
!define rdx [24:8]
!define result [32:8]

; Set rax to start
; Set rbx to end
.printbinary
	ext printnum, [rax]
	add rax, rax, 1
	jne rax, rbx, printbinary
	ret

.start
	ext print, "\n"
	
	mov rcx, b11001100
	mov rax, <rcx
	mov rbx, >rcx
	ext print, "Bit one: "
	call printbinary
	ext print, "\n"

	mov rdx, b01010101
	mov rax, <rdx
	mov rbx, >rdx
	ext print, "Bit two: "
	call printbinary
	ext print, "\n\n"

	and result, rcx, rdx
	mov rax, <result
	mov rbx, >result
	ext print, "And:     "
	call printbinary
	ext print, "\n"

	or  result, rcx, rdx
	mov rax, <result
	mov rbx, >result
	ext print, "Or:      "
	call printbinary
	ext print, "\n"

	xor result, rcx, rdx
	mov rax, <result
	mov rbx, >result
	ext print, "Xor:     "
	call printbinary
	ext print, "\n"
