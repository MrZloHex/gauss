use std::fs::{metadata, File};
use std::io::{Read, Write};

// use ron::de::from_reader;

// use crate::types::*;

pub fn load_file(filename: &String) -> String {
    let mut file = match File::open(filename) {
        Err(why) => {
            eprintln!("ERROR: couldn't open {}: {}", filename, why);
            std::process::exit(1);
        },
        Ok(file) => file,
    };

    let mut code = String::new();
    match file.read_to_string(&mut code) {
        Err(why) => {
            eprintln!("ERROR: couldn't read {}: {}", filename, why);
            std::process::exit(1);
        },
        Ok(_) => {}
    }

    code
}

pub fn store_file(content: String, filename: String) {
    let mut f = File::create(&filename).expect("can't create file");
    f.write_all(content.as_bytes())
        .expect("failed to write into file");
}

//pub fn store_ron(functions: Vec<Function>, filename: String) {
//    let pretty = PrettyConfig::new()
//        .with_new_line("\n".to_owned())
//        .with_indentor("\t".to_owned());
//    let content = to_string_pretty(&functions, pretty).expect("Serialization failed");
//    write(filename, content).expect("Unable to write file");
//}

// pub fn load_ofs(filename: String) -> Vec<Function> {
//     let file = File::open(filename).expect("Failed opening file");
//     let functions: Vec<Function> = match from_reader(file) {
//         Ok(f) => f,
//         Err(e) => {
//             eprintln!("Failed to read OFS, {}", e);
//             std::process::exit(1);
//         }
//     };
//
//     functions
// }
