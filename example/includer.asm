jmp start
!include example/included.asm

.start
	call included_function
	ext print, "Included another file!\n"
