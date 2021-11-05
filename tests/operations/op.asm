SECTION .bss
	_2number:	resw 1
	_ressum:	resb 1

SECTION .data
	_1number:	db 34

SECTION .text
	_func420_:
		push rbp
		mov  rbp,	rsp
		sub  rsp,	3
		mov  rax, QWORD [rbp+16]
		mov  BYTE [rbp-1], al
		mov  WORD [rbp-3],	420
		mov  rax,	 0
		mov  ax,	WORD [rbp-3]
		leave
		ret
	global _start
	_start:
		; Assigning value `35` to variable `2number`
		mov	BYTE [_2number], 35
		; Assigning result expresion to variable `ressum`
		mov	al, BYTE [_1number]
		mov	bx, WORD [_2number]
		add	al, bl
		; Assigning result expresion to variable `1number`
		mov	ax, 2345
		mov	bx, 1234
		mul	bl

		; Exit syscall
		mov	rax, 0x3c
		mov	rdi, 0
		syscall

