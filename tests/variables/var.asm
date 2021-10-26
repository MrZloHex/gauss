SECTION .bss
	var1_:	resb 1
	var2_:	resw 1
	var5_:	resw 1

SECTION .data
	var3_:	db 69
	var4_:	dw 420

SECTION .text
	global _start
	_start:
		; Assigning variable `var3` to variable `var1`
		mov  al, BYTE [var3_]
		mov  BYTE [var1_], al
		; Assigning value `420` to variable `var2`
		mov	WORD [var2_], 420
		; Assigning value `60` to variable `var5`
		mov	BYTE [var5_], 60

		; Exit syscall
		mov	rax, 0x3c
		mov	rdi, 0
		syscall

