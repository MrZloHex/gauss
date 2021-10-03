use std::fs;
use std::io::Read;

use ron::ser::{to_string_pretty, PrettyConfig};

use crate::instr::*;

pub fn load_file(filename: String) -> Vec<u8> {
    let mut f = fs::File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

pub fn store_ron(functions: Vec<Function>, filename: String) {
    let pretty = PrettyConfig::new()
        .with_new_line("\n".to_owned())
        .with_indentor("\t".to_owned());
    let content = to_string_pretty(&functions, pretty).expect("Serialization failed");
    fs::write(filename, content).expect("Unable to write file");
}

