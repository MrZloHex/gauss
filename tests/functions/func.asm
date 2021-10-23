SECTION .bss
	eight2_:	resb 1
	var_:	resw 1
	eight1_:	resb 1

SECTION .data
	myvar_:	dw 420

SECTION .text
	eight_:
		push rbp
		mov  rbp,	rsp
		sub  rsp,	3
		mov  BYTE [rbp-1],	8
		mov  WORD [rbp-3],	2345
		mov  rax,	 0
		mov  al,	BYTE [rbp-1]
		leave
		ret
	five_:
		push rbp
		mov  rbp,	rsp
		sub  rsp,	1
		mov  BYTE [rbp-1],	5
		mov  rax,	 0
		mov  al,	BYTE [rbp-1]
		leave
		ret
	global _start
	_start:
		mov	BYTE [eight2_], 69
		mov	ax, WORD [myvar_]
		mov	WORD [var_], ax
		mov	BYTE [eight1_], 0

		mov	rax, 0x3c
		mov	rdi, 0
		syscall

