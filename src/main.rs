use clap::{load_yaml, App};
use std::process::Command;
use std::path::Path;

mod instr;

mod file;
use file::load_file;

mod lexer;
use lexer::parse_code;

mod translator;
use translator::generate_assembler;

fn main() {
    // Allocating memory for files' names
    let mut input_filename: String = String::new();
    let mut output_filename: String = String::new();
    let mut filename: String = String::new();
    let mut asm_filename: String = String::new();
    let mut object_filename: String = String::new();
    
    let yaml = load_yaml!("cli.yaml");
    let cli = App::from_yaml(yaml).get_matches();

    // COMPILE
    if let Some(matches) = cli.subcommand_matches("compile") {
        if matches.is_present("input") {
            if let Some(in_f) = matches.value_of("input") {
                input_filename = in_f.to_string();
            }
        }
        if matches.is_present("output") {
            if let Some(out_f) = matches.value_of("output") {
                output_filename = out_f.to_string();
            };
        } else {
            output_filename = input_filename.replace(".ris", "");
        }
    }
    filename = input_filename.replace(".ris", "");
    asm_filename = input_filename.replace(".ris", ".asm");
    object_filename = input_filename.replace(".ris", ".o");

    let main_is = parse_code(load_file(input_filename));
    generate_assembler(main_is, asm_filename.clone());

    let mut build = Command::new("nasm");
    build.arg("-felf64");
    build.arg(asm_filename);
    build.arg("-o");
    build.arg(object_filename.clone());
    build.spawn();

    let mut link = Command::new("ld");
    link.arg(object_filename);
    link.arg("-o");
    link.arg(output_filename);
    link.spawn();
}

