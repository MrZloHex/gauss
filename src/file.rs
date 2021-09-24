use std::fs::File;
use std::io::{BufRead, BufReader};



pub fn load_file(filename: String) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut data: Vec<String> = Vec::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        data.push(line);
    }
    data
}
