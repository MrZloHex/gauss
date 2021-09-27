use clap::{load_yaml, App};
use std::process::Command;
use std::path::Path;

mod instr;

mod file;
use file::load_file;

mod lexer;
use lexer::lex_code;

fn main() {
    // Allocating memory for files' names
    let mut input_filename: String = String::new();
    let mut object_filename: String = String::new();
    
    let yaml = load_yaml!("cli.yaml");
    let cli = App::from_yaml(yaml).get_matches();

    // COMPILE
    if let Some(matches) = cli.value_of("input") {
        input_filename = matches.to_string();
    }

    let smt: Vec<&str> = input_filename.split('.').collect();
    if smt[smt.len()-1] != "gfs" {
        eprintln!("Functions set should be with extension '.gfs'");
        std::process::exit(1);
    }

    object_filename = input_filename.replace(".gfs", ".gofs");

    let code = load_file(input_filename);
    let func_or = lex_code(code);

    //let main_is = parse_code(load_file(input_filename));
    //generate_assembler(main_is, asm_filename.clone());

    //let mut build = Command::new("nasm");
    //build.arg("-felf64");
    //build.arg(asm_filename);
    //build.arg("-o");
    //build.arg(object_filename.clone());
    //build.spawn();

    //let mut link = Command::new("ld");
    //link.arg(object_filename);
    //link.arg("-o");
    //link.arg(output_filename);
    //link.spawn();
}

