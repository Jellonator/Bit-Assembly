push 512

jmp begin
.error
ext print, "Error: same number\n"

.begin

ext print, "Input minimum number: "
ext prompt
ext inputnum, [0:64] ;Can't be bothered to validate this input

ext print, "Input maximum number: "
ext prompt
ext inputnum, [64:64]

sub [64:64], [64:64], [0:64]

je [64:64], 0, error

mov [256:64], 10

.again
	mov [192:64], [64:64]
	ext random, [192:64]
	add [128:64], [192:64], [0:64]

	ext printnum, [128:64]
	ext print, ","

	sub [256:64], [256:64], 1

	jne [256:64], 0, again

ext print, "\n"
jmp begin
