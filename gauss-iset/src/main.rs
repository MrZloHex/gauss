use clap::{load_yaml, App};

mod instr;

mod file;
use file::*;

//mod lexer;
//use lexer::lex_code;

fn main() {
    // Allocating memory for files' names
    let mut is_filename: String = String::new();
    let mut ofs_filename: String = String::new();
    
    let yaml = load_yaml!("cli.yaml");
    let cli = App::from_yaml(yaml).get_matches();

    // COMPILE
    if let Some(matches) = cli.value_of("is") {
        is_filename = matches.to_string();
    }

    let smt: Vec<&str> = is_filename.split('.').collect();
    if smt[smt.len()-1] != "gis" {
        eprintln!("Functions set should be with extension '.gis'");
        std::process::exit(1);
    }

    if let Some(matches) = cli.value_of("ofs") {
        ofs_filename = matches.to_string();
    }

    let smt: Vec<&str> = ofs_filename.split('.').collect();
    if smt[smt.len()-1] != "gofs" {
        eprintln!("Functions set should be with extension '.gofs'");
        std::process::exit(1);
    }

    let functions = load_ofs(ofs_filename);
    for function in functions {
        println!("{:?}", function)
    }
    //let code = load_file(input_filename);
    //let func_or = lex_code(code);
    //print!("{}", func_or);
    //store_file(func_or, object_filename);
}

