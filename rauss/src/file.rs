use std::fs::{metadata, File};
use std::io::Read;

// use ron::de::from_reader;

// use crate::types::*;

pub fn load_file(filename: String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
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

