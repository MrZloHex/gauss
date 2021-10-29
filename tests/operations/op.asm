SECTION .bss
	_2number:	resb 1
	_ressum:	resb 1

SECTION .data
	_1number:	db 34

SECTION .text
	global _start
	_start:
		; Assigning value `35` to variable `2number`
		mov	BYTE [_2number], 35

		; Exit syscall
		mov	rax, 0x3c
		mov	rdi, 0
		syscall

