section .bss
	heap: RESB 8
section .text
	global _start
	_start:
		; == STORE ==
		mov rax, 32
		mov rbx, 0
		mov [heap+rbx], rax
		; == STORE ==
		mov rax, 34
		mov rbx, 4
		mov [heap+rbx], rax
	_exit:
		xor rdi, rdi
		mov rax, 0
		syscall
