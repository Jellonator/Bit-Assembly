push 512
!define top @[-0]
!define rax [0:64]
!define rbx [64:64]
!define rcx [128:64]
!define rdx [192:64]
!define rex [256:64]

.first-start
	ext print, "Input first number to add: "
	ext prompt
	ext inputnumlen, rax
	ext valid, rbx
	je rbx, 0, first-fail
	jmp first-good
.first-fail
	ext print, "Invalid number!\n"
	jmp first-start
.first-good
	mov rbx, top
	push rax
	ext inputnum, [rbx:rax]
	ext print, "You input: "
	ext printnum, [rbx:rax]
	ext print, "\n"

.second-start
	ext print, "Input first number to add: "
	ext prompt
	ext inputnumlen, rcx
	ext valid, rdx
	je rdx, 0, second-fail
	jmp second-good
.second-fail
	ext print, "Invalid number!\n"
	jmp second-start
.second-good
	mov rdx, top
	push rcx
	ext inputnum, [rdx:rcx]
	ext print, "You input: "
	ext printnum, [rdx:rcx]
	ext print, "\n"

jg rax, rcx, comp-large
jmp comp-small
.comp-small
	mov rex, rcx
	jmp comp-end
.comp-large
	mov rex, rax
	jmp comp-end
.comp-end
add rex, rex, 1

push rex
add [-rex:rex], [rbx:rax], [rdx:rcx]
ext print, "Result: "
ext printnum, [-rex:rex]
ext print, "\nBits used: "
ext printnum, rex
ext print, "\n"
pop rex
pop rcx
pop rax
