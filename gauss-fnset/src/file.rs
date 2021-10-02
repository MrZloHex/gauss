use std::fs;
use std::io::Read;



pub fn load_file(filename: String) -> Vec<u8> {
    let mut f = fs::File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

pub fn store_file(content: String, filename: String) {
    fs::write(filename, content).expect("Unable to write file");
}

