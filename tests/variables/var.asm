SECTION .bss
	_var1:	resb 1
	_var2:	resw 1
	_var5:	resw 1

SECTION .data
	_var3:	db 69
	_var4:	dw 420

SECTION .text
	global _start
	_start:
		; Assigning variable `var3` to variable `var1`
		mov  al, BYTE [_var3]
		mov  BYTE [_var1], al
		; Assigning value `420` to variable `var2`
		mov	WORD [_var2], 420
		; Assigning value `60` to variable `var5`
		mov	BYTE [_var5], 60

		; Exit syscall
		mov	rax, 0x3c
		mov	rdi, 0
		syscall

