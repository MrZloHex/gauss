#!/bin/python

import subprocess
import os


test_dirs = ["tests/functions/func.gis", "tests/variables/var.gis"]
test_files = {
    "tests/functions/func.gis": "tests/functions/func.asm",
    "tests/variables/var.gis": "tests/variables/var.asm"
}

test_source = {
    "tests/functions/func.asm": """SECTION .bss
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

""",
    "tests/variables/var.asm": """SECTION .bss
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

"""
}

def compiler_recompile():
    os.chdir("rauss")
    rauss= subprocess.run(["cargo","build","--release"])
    os.chdir("..")
    if rauss.returncode != 0:
        print("Can't compile compiler")
        exit(1)


def test(test_dir: str) -> bool:
    res = subprocess.run(["./rauss/target/release/rauss","--input",test_dir])
    if res.returncode != 0:
        print(f"FAILED TO COMPILE SOURCE OF TEST {test_dir}")
        exit(res.returncode)

    test_asm = test_files[test_dir]
    with open(test_asm, "r") as f:
        test_output = f.read()

    if test_output == test_source[test_asm]:
        return True
    else:
        return False

def main():
    compiler_recompile()
    for test_dir in test_dirs:
        if test(test_dir):
            print(f"TEST PASSED:   {test_dir}")
        else:
            print(f"TEST FAILED:   {test_dir}")

if __name__ == "__main__":
    main()