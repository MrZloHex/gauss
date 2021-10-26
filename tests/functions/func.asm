SECTION .bss
	_eight2:	resb 1
	_var:	resw 1
	_eight1:	resb 1
	_five1:	resw 1

SECTION .data
	_myvar:	dw 420

SECTION .text
	_eight_:
		push rbp
		mov  rbp,	rsp
		sub  rsp,	3
		mov  BYTE [rbp-1],	8
		mov  WORD [rbp-3],	2345
		mov  rax,	 0
		mov  al,	BYTE [rbp-1]
		leave
		ret
	_retsame_:
		push rbp
		mov  rbp,	rsp
		sub  rsp,	2
		mov  rax, QWORD [rbp+16]
		mov  WORD [rbp-2], ax
		mov  rax,	 0
		mov  ax,	WORD [rbp-2]
		leave
		ret
	_five_:
		push rbp
		mov  rbp,	rsp
		sub  rsp,	4
		mov  rax, QWORD [rbp+16]
		mov  BYTE [rbp-1], al
		mov  rax, QWORD [rbp+24]
		mov  WORD [rbp-3], ax
		mov  BYTE [rbp-4],	5
		mov  rax,	 0
		mov  al,	BYTE [rbp-4]
		leave
		ret
	global _start
	_start:
		; Assigning value `69` to variable `eight2`
		mov	BYTE [_eight2], 69
		; Assigning variable `myvar` to variable `var`
		mov  ax, WORD [_myvar]
		mov  WORD [_var], ax
		; Assigning result of function `eight` to variable `eight1`
		call _eight_
		add  rsp, 8 * 0
		mov	BYTE [_eight1], al
		; Assigning result of function `retsame` to variable `five1`
		push 42
		call _retsame_
		add  rsp, 8 * 1
		mov	WORD [_five1], ax

		; Exit syscall
		mov	rax, 0x3c
		mov	rdi, 0
		syscall

