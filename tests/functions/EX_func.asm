; It is somewhere about what should output gauss-compiler

section .text
	eight_:
		push rbp
		mov  rbp, rsp

		mov  rax, 8

		pop  rbp
		ret
	retsame_:
		push rbp
		mov  rbp, rsp
		sub  rsp, 2
		
		mov  ax, di
		mov  WORD [rbp-2], ax
		mov  ax, WORD [rbp-2]
		
		leave
		ret

	five_:
		push rbp
		mov  rbp, rsp
		sub  rsp, 1
		
		mov  rax, 5
		mov  BYTE [rbp-1], al
		mov  al, BYTE [rbp-1]

		leave
		ret
	
	global _start
	_start:
		call eight_
		mov  [eight1], al


	_stop:
		mov  rdi, 0
		mov  rax, 0x3C
		syscall
section .data
	myvar dw 1234

section .bss
	eight1 resb 1
