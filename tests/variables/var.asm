SECTION .bss
	var1_:	resb 1
	var2_:	resw 1

SECTION .data
	var3_:	db 69
	var4_:	dw 420

SECTION .text
	global _start
	_start:
		mov	al, BYTE [var3_]
		mov	BYTE [var1_], al
		mov	WORD [var2_], 420

		mov	rax, 0x3c
		mov	rdi, 0
		syscall

