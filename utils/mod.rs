use std::fs::File;
use std::io::prelude::*;

pub fn read_input_lines(f: &str) -> Vec<String> {
    let mut file = File::open(f).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
}
