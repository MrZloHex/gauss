SECTION .bss
	eight2_:	resb 1
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
		leave
		ret
	retsame_:
		push rbp
		mov  rbp,	rsp
		pop  rbp
		ret
	five_:
		push rbp
		mov  rbp,	rsp
		sub  rsp,	1
		mov  BYTE [rbp-1],	5
		leave
		ret
	global _start
	_start:
		mov	BYTE [eight2_], 69
		mov	al, BYTE [eight2_]
		mov	BYTE [eight1_], al

		mov	rax, 0x3c
		mov	rdi, 0
		syscall

