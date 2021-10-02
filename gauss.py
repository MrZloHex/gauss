#!/bin/python

import sys
import subprocess
import os

def parse_cli() -> str:
    if len(sys.argv) == 1:
        assert False, "HELP CLI OPTIONS"

    argc = sys.argv[1:]
    if argc[0] == "--help":
        assert False, "HELP CLI OPTIONS"
    else:
        make_fn = str(argc[0])

    return make_fn


def load_file(filename: str) -> list:
    with open(filename, "r") as file:
        return file.readlines()

def spawn_func(func_set: str):
    subprocess.run(["./gauss-fnset/target/release/gauss-fnset","--input",func_set])

def spawn_instr(instr_set: str, of_set: str):
    subprocess.run(["./gauss-iset/target/release/gauss-iset","--is",instr_set,"--ofs",of_set])

def parse_instr(code: list):
    for line in code:
        tokens = line.split()
        if tokens[0] == "function":
            func_set = tokens[1]
            spawn_func(func_set)
        elif tokens[0] == "compile":
            instr_set = tokens[1]
            of_set = tokens[2]
            spawn_instr(instr_set, of_set)
        else:
            assert False, "Uniplemented method"

def precompile():
    os.chdir("gauss-fnset")
    fnset = subprocess.run(["cargo","build","--release"])
    os.chdir("..")
    os.chdir("gauss-iset")
    iset = subprocess.run(["cargo","build","--release"])
    os.chdir("..")
    if fnset.returncode != 0 or iset.returncode != 0:
        print("Can't compile compiler")
        exit(1)


def main():
    precompile()
    make_fn = parse_cli()
    parse_instr(load_file(make_fn))


if __name__ == "__main__":
    main()

