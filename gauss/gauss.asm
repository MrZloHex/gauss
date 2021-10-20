; GAUSS COMPILER BUT MANUALY COMPILED
extern _malloc

section .text
	display_:
		push rbp
		mov  rbp, rsp
		sub  rsp, 16

		mov  QWORD [rbp-8],  rdi
		mov  WORD  [rbp-10], si

		mov  BYTE  [rbp-11], 1
		mov  BYTE  [rbp-12], 1

		mov  rax, 0
		mov  rdi, 0
		mov  rsi, 0
		mov  rdx, 0
		mov  al,  BYTE  [rbp-11]
		mov  dil, BYTE  [rbp-12]
		mov  rsi, QWORD [rbp-8]
		mov  dx,  WORD  [rbp-10]
		syscall

		leave
		ret

	exit_:
		push rbp
		mov  rbp, rsp
		sub  rsp, 16

		mov  BYTE [rbp-1], dil

		mov  BYTE [rbp-2], 60

		mov  rax, 0
		mov  rdi, 0
		mov  al,  BYTE [rbp-2]
		mov  dil, BYTE [rbp-1]
		syscall

		leave
		ret
	
	global _start
	_start:
		mov  rdi, hello_
		mov  rsi, 13
		call display_

		mov  rdi, 0
		call exit_

		
section .data
	hello_ db "Hello World", 10, 0

section .bss
