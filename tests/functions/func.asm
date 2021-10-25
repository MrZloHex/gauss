SECTION .bss
	eight2_:	resb 1
	var_:	resw 1
	eight1_:	resb 1
	five1_:	resb 1

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
		; Assigning value `69` to variable `eight2`
		mov	BYTE [eight2_], 69
		; Assigning variable `myvar` to variable `var`
		mov  ax, WORD [myvar_]
		mov  WORD [var_], ax
		; Assigning result of function `eight` to variable `eight1`
		call eight_
		add  rsp, 8 * 0
		mov	BYTE [eight1_], al
		; Assigning result of function `five` to variable `five1`
		call eight_
		add  rsp, 8 * 2
		push rax
		push 420
		call five_
		add  rsp, 8 * 2
		push rax
		mov  ax, WORD [var_]
		push rax
		call five_
		add  rsp, 8 * 2
		mov	BYTE [five1_], al

		; Exit syscall
		mov	rax, 0x3c
		mov	rdi, 0
		syscall

