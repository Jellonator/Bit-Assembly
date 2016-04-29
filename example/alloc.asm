push 1024 ; bits for registers

; Defines are NOT variables, but they can represent a position on the stack
; and behave like one though.

!define top <[-0] ; top of stack
!define rax [  0:64] ; register a
!define rbx [ 64:64] ; register b
!define rcx [128:64] ; register c
!define rdx [192:64] ; register d
!define debug_num [256:64] ; debugging number I guess

!define allocate_chunk_size 64 ; 64 bits per chunk, could easily be any other value
!define allocated_space 4096   ; allocate a total of 4 kb, or 64 chunks / 512 Bytes

!define allocate_pos [512:64] ; starting position of heap
!define allocate_len [576:64] ; length of heap
!define metadata_pos [640:64] ; starting position of metadata ( bookkeaping )
!define metadata_len [704:64] ; length of metadata ( size of heap / size of chunk )
!define metadata_ptr [768:64] ; pointer used inside allocator function

mov allocate_len, allocated_space ; set length of allocater to constant
mov allocate_pos, top ; position of heap to top of stack
push allocate_len     ; reserve space for heap ( not how an actual heap works )

; set size of metadata to length of heap space divided by the size of a chunk
div metadata_len, allocate_len, allocate_chunk_size
mov metadata_pos, top ; position of metadata is now top of stack
push metadata_len     ; push length of metadata to the stack

mov debug_num, 1 ; set debugging number to 1

; How to use allocate function:
; Set rax to number of bits needed for allocation
; rbx will be set to the first position of the allocated bits
jmp start
.allocate
	mov rdx, rax ; set rdx to the size in bits

	; We need to get rax to refer to chunks now, divide by chunk size round up
	mod rcx, rax, allocate_chunk_size
	div rax, rax, allocate_chunk_size

	; one weird trick to round up! (add one if it doesn't divide evenly)
	je rcx, 0, allocate_not_need_add
	add rax, rax, 1
	.allocate_not_need_add

	mov metadata_ptr, metadata_pos ; set pointer to start of heap
	; Inefficient, but works
	.allocate_loop_start
		; Loop through every possible heap position and test if it is free
		je [metadata_ptr:rax], 0, allocate_loop_end ; test if free
		add metadata_ptr, metadata_ptr, 1 ; next position
		jmp allocate_loop_start           ; try again
	.allocate_loop_end
	; We in bizniz now

	mov [metadata_ptr:rax], 0 ; Set metadata to owned
	not [metadata_ptr:rax], [metadata_ptr:rax] ; Set metadata to owned

	mov rbx, metadata_ptr      ; Set rbx to metadata pointer
	sub rbx, rbx, metadata_pos ; Subtract metadata position
	mul rbx, rbx, allocate_chunk_size ; Multiply by size of chunk
	add rbx, rbx, allocate_pos ; Add allocation position to rbx
	mov [rbx:rdx], 0           ; Clear allocated memory

	; For debugging purposes, deallocated bytes are going to be a non-zero
	; number refering to when it was allocated
	mov rcx, rbx     ; rcx is start
	add rdx, rdx, rcx; rdx is end
	.allocate_debug_loop_start
		mov [rcx:allocate_chunk_size], debug_num
		add rcx, rcx, allocate_chunk_size
		jl rcx, rdx, allocate_debug_loop_start
	.allocate_debug_loop_end
	add debug_num, debug_num, 1

	ret ; rbx now corresponds to position in heap!

; How to use deallocate function:
; Set rax to position of bits for deallocation
; Set rbx to size of bits for deallocation
.deallocate
	; For debugging purposes, deallocated bytes are going to be reset to '0'
	mov rdx, rax
	mov rcx, rax      ; rcx is start
	add rdx, rdx, rbx ; rdx is end
	.deallocate_debug_loop_start
		mov [rcx:allocate_chunk_size], 0  ; set chunk to 0
		add rcx, rcx, allocate_chunk_size ; go to next chunk
		jl rcx, rdx, deallocate_debug_loop_start ; loop again
	.deallocate_debug_loop_end

	; We need to get rbx to refer to bytes now
	mod rcx, rbx, allocate_chunk_size
	div rbx, rbx, allocate_chunk_size

	; Using the 'weird' trick again
	je rcx, 0, deallocate_not_need_add
	add rbx, rbx, 1
	.deallocate_not_need_add

	; Now rax needs to refer to metadata position
	sub rax, rax, allocate_pos
	div rax, rax, allocate_chunk_size
	add rax, rax, metadata_pos

	; Now clear metadata
	mov [rax:rbx], 0
	ret

.start
	push 256

	;allocate 64 bits (1 chunk)
	mov rax, 64       ; We want 64 bits
	call allocate     ; call the allocater
	mov [-64:64], rbx ; Store address of allocated memory
	ext print, "Allocated 64 bits to "
	ext printnum, [-64:64] ; Print out adress

	; allocate 200 bits (4 chunks)
	mov rax, 200 ; Now we want 200 bits, do the whole shagrin again
	call allocate
	mov [-128:64], rbx
	ext print, "\nAllocated 200 bits to "
	ext printnum, [-128:64]

	; allocate 300 bits (5 chunks)
	mov rax, 300 ; Now we want 300 bits
	call allocate
	mov [-192:64], rbx
	ext print, "\nAllocated 300 bits to "
	ext printnum, [-192:64]

	; deallocate previously allocated 200 bits (free 4 chunks)
	mov rax, [-128:64] ; This is the position
	mov rbx, 200       ; And this is the size
	call deallocate    ; Call the deallocation function
	ext print, "\nDeallocated 200 bits"

	;allocate 64 bits (1 chunk)
	mov rax, 64   ; we want another 64 bits
	call allocate ; It should place this into the bits we freed earier
	ext print, "\nAllocated 64 bits to "
	ext printnum, rbx

	;allocate 64 bits (1 chunk)
	mov rax, 64
	call allocate
	ext print, "\nAllocated 64 bits to "
	ext printnum, rbx

	;allocate 200 bits (4 chunks)
	mov rax, 200
	call allocate
	ext print, "\nAllocated 200 bits to "
	ext printnum, rbx

	;allocate 200 bits (4 chunks)
	mov rax, 200
	call allocate
	ext print, "\nAllocated 200 bits to "
	ext printnum, rbx

	;allocate 200 bits (4 chunks)
	mov rax, 200
	call allocate
	ext print, "\nAllocated 200 bits to "
	ext printnum, rbx

	ext print, "\n"
