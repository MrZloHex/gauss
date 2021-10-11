#!/bin/python

import sys
import subprocess
import os
import fnmatch

def parse_cli() -> (bool, str):
    if len(sys.argv) > 1:
        return (True, sys.argv[1])
    else:
        return (False, "")


def load_file(filename: str) -> list:
    with open(filename, "r") as file:
        return file.readlines()

def spawn_compiler(GIS: str):
    GISC = subprocess.run(["./gauss-iset/target/release/gauss-iset","--input",GIS])
    exit(GISC.returncode)

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
        if tokens[0] == "build":
            # TODO: Implement manual defining path to .gis file
            GIS = flnm.replace(".gbi", ".gis")
            if search_file(GIS):
                spawn_compiler(GIS)
            else:
                print(f"Didn't find {GIS}")
                exit(1)
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
