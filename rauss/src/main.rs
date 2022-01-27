use std::char::from_u32_unchecked;

use clap::{load_yaml, App};

mod types;

mod file;
use file::*;

mod lexer;
use lexer::Lexer;

mod analyzer;
use analyzer::{
    //analyze_direct,
    analyze_func,
    analyze_instr,
};

mod compile;
use compile::into_nasm;

// mod preproc;
// use preproc::pre_proc_direct;

fn main() {
    // Allocating memory for files' names
    let mut is_filename: String = String::new();
    let out_filename: String;

    let mut is_functions = false;
    let mut filenames_func: Vec<types::Indent> = Vec::new();
    let mut functions: Vec<types::Function> = Vec::new();

    let yaml = load_yaml!("cli.yaml");
    let cli = App::from_yaml(yaml).get_matches();

    // COMPILE
    if let Some(matches) = cli.value_of("input") {
        is_filename = matches.to_string();
    };

    if !check_ext(is_filename.clone(), 0) {
        std::process::exit(1)
    }
    out_filename = is_filename.replace(".gis", ".asm");

    // get source code  
    let code = load_file(&is_filename);
    println!("{}", code);

    let mut lexer = Lexer::new(code, is_filename);
    lexer.lex();
    
}



fn check_ext(filename: String, type_ext: u8) -> bool {
    let smt: Vec<&str> = filename.split('.').collect();
    match type_ext {
        0 => {
            if smt[smt.len() - 1] != "gis" {
                eprintln!("Instruction set should be with extension '.gis'");
                std::process::exit(1);
            } else {
                return true;
            }
        }
        1 => {
            if smt[smt.len() - 1] != "gfs" {
                eprintln!("Functions set should be with extension '.gfs'");
                std::process::exit(1);
            } else {
                return true;
            }
        }
        _ => unreachable!(),
    }
}
