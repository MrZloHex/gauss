use clap::{load_yaml, App};

mod types;

mod file;
use file::*;

mod lexer;
use lexer::{lex_direct, lex_func, lex_instr};

mod analyzer;
use analyzer::{
    //analyze_direct,
    analyze_func,
    analyze_instr,
};

mod compile;
use compile::into_nasm;

mod preproc;
use preproc::pre_proc_direct;

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
    let mut code = load_file(is_filename);
    
    // get directives
    let directives = lex_direct(code.clone());
    let mut is_cli_arguments = false;
    let mut arguments = (types::Indent(String::new()), types::Indent(String::new()));
    for directive in &directives {
        match (*directive).clone() {
            types::Directive::Use(mut files_i) => {
                is_functions = true;
                filenames_func.append(&mut files_i);
            },
            types::Directive::Args(args) => {
                is_cli_arguments = true;
                arguments = args;
            },
            _ => (),
        }
    }
    pre_proc_direct(&directives, &mut code);
    println!("");
    for s in code {
        print!("{}", s as char)
    }
    println!("");
    // get directives from function sets
    // if is_functions {
    //     for fs in filenames_func {
    //         let fs_filename = fs.0.clone();
    //         if !check_ext(fs_filename.clone(), 1) {
    //             std::process::exit(1)
    //         }
    //         let function_code = load_file(fs_filename);
    //         let func_directives = lex_direct(function_code.clone());
    //         for f_d in &func_directives { println!("{:?}", f_d) }
    //         pre_proc_direct(&func_directives, &function_code);
    //         // functions.append(&mut lex_func(functions_code));
    //     }
    //     // for func in &functions { println!("{:?}", func) }
    //     if !analyze_func(&functions) {
    //         eprintln!("\nFAILED TO CHECK FUNCTIONS");
    //         std::process::exit(1);
    //     }
    // }



    // get set of instructi0ns
    // let instructions = lex_instr(code);
    // for instr in &instructions {
    // println!("{:?}", instr);
    // }

    // let (ok, variables) = analyze_instr(&instructions, &functions, &arguments);
    // if !ok {
    //     eprintln!("\nFAILED TO CHECK");
    //     std::process::exit(1);
    // }

    // let nasm = into_nasm(instructions, variables, functions);
    // store_file(nasm, "gauss.asm".to_string())

    // let program = Program {
    //     instructions,
    //     variables,
    //     functions
    // };
    // let gos = ron::to_string(&program).expect("can't");
    // store_file(gos, "gauss.gos".to_string());

    // TODO
    // if let Some(directives) = directives_o {
    //     for directive in directives {
    //         println!("{:?}", directive);
    //         // match directive {
    //         //     Directive::Use(filnames) =>
    //         // }
    //     }
    // }
}
#[derive(serde::Serialize)]
struct Program {
    instructions: Vec<types::Instruction>,
    variables:    Vec<types::Variable>,
    functions:    Vec<types::Function>
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
