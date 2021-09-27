use std::fs::File;
use std::io::prelude::*;

use crate::instr::*;

pub fn generate_assembler(is: (Vec<Instruction>, ProgrammInfo), filename: String) {
    let mut code: String = String::new();
    code.push_str(        "section .bss\n");
    code.push_str(format!("\theap: RESB {}\n", is.1.heap_vol.clone()).as_str());
    code.push_str(        "section .text\n");
    code.push_str(        "\tglobal _start\n");
    code.push_str(        "\t_start:\n");

    for instruction in is.0 {
        match instruction.op {
            Operation::Store(store) => {
                code.push_str(        "\t\t; == STORE ==\n");
                code.push_str(format!("\t\tmov rax, {}\n", store.value).as_str());
                code.push_str(format!("\t\tmov rbx, {}\n", store.address).as_str());
                match store.loc {
                   Location::Heap => code.push_str("\t\tmov [heap+rbx], rax\n"),
                }
            },
            Operation::Set(set) => {
                code.push_str(        "\t\t; == SET ==\n");
                code.push_str(format!("\t\tmov rax, {}\n", set.value).as_str());
                code.push_str(        "\t\tmov [heap], 
            }
        }
    }

    code.push_str(        "\t_exit:\n");
    code.push_str(        "\t\txor rdi, rdi\n");
    code.push_str(        "\t\tmov rax, 0\n");
    code.push_str(        "\t\tsyscall\n");
    store_file(code, filename);
}

fn store_file(code: String, filename: String) {
    let mut file = File::create(filename).unwrap();
    file.write_all(code.as_bytes()).unwrap();
}

