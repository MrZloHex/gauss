SECTION .bss
	eight2_:	resb 1
	eight1_:	resb 1

SECTION .data
	myvar_:	dw 420

SECTION .text
	global _start
	_start:
		mov	BYTE [eight2_], 69
		mov	al, BYTE [eight2_]
		mov	BYTE [eight1_], al

		mov	rax, 0x3c
		mov	rdi, 0
		syscall

