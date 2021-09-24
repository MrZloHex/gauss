use clap::{load_yaml, App};

mod instr;

mod file;
use file::load_file;

mod lexer;
use lexer::parse_instr;

fn main() {
    // Allocating memory for files' names
    let mut input_filename: String = String::new();
    let mut output_filename: String = String::new();
    
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

    let main_is = parse_instr(load_file(input_filename));
}

