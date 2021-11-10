#!/bin/python

import sys
import subprocess
import os

def parse_cli() -> tuple:
    if len(sys.argv) > 1:
        return (True, sys.argv[1])
    else:
        return (False, "")


def load_file(filename: str) -> list:
    with open(filename, "r") as file:
        return file.readlines()

def spawn_compiler(GIS: str):
    print("COMPILING")
    cmd = ["cd", "/".join(GIS.split('/')[:-1]), "&&", "exec", "/home/zs/gauss/rauss/target/release/rauss","--input",GIS.split('/')[-1] ]
    GISC = subprocess.run(" ".join(cmd), shell=True)
    if GISC.returncode != 0:
        exit(GISC.returncode)
    subprocess.run(["nasm","-felf64",GIS.replace(".gis", ".asm"),"-o",GIS.replace(".gis", ".o")])
    print("LINKING")
    subprocess.run(["ld", GIS.replace(".gis", ".o"),"-o", GIS.replace(".gis", "")])
    print("FINISHED")

def search_file(filename: str) -> bool:
    result = False
    path = os.path.dirname(os.path.abspath(filename))
    for root, dirs, files in os.walk(path):
        for name in files:
            if name == os.path.basename(filename):
                result = True
                return result
    return result

def parse_instr(code: list, flnm: str):
    for line in code:
        tokens = line.split()
        if not tokens:
            continue
        if tokens[0] == "test":
            subprocess.run(["./test.py"])
        elif tokens[0] == "build":
            # TODO: Implement manual defining path to .gis file
            GIS = flnm.replace(".gbi", ".gis")
            if search_file(GIS):
                spawn_compiler(GIS)
            else:
                print(f"Didn't find {GIS}")
                exit(1)
        elif tokens[0] == "run":
            print("RUNNING")
            executable = flnm.replace(".gbi", "")
            subprocess.run([f"./{executable}"])
        else:
            print(tokens)
            assert False, "Uniplemented method"

def precompile():
    os.chdir("rauss")
    rauss= subprocess.run(["cargo","build","--release"])
    os.chdir("..")
    if rauss.returncode != 0:
        print("Can't compile compiler")
        exit(1)


def main():
    precompile()
    (isGBI, NameGBI) = parse_cli()
    if isGBI:
        GBI = load_file(NameGBI)
        parse_instr(GBI, NameGBI)
    # TODO: SEARCH FOR MAKE IN DIRECTORY 
    # https://stackoverflow.com/questions/1724693/find-a-file-in-python
    #else:
    #    (isGBI, NameGBI) = search_gbi()


if __name__ == "__main__":
    main()
