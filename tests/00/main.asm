section .bss
	heap: RESB 8
section .text
	global _start
	_start:
		; == STORE ==
		mov rax, 0
		mov rbx, 0
		mov [heap+0], al
		mov rax, 255
		mov [heap+1], al
		mov rax, 0
		mov ax, [heap+0]
	_exit:
		xor rdi, rdi
		mov rax, 0
		syscall
