use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_input_lines(dir: &str) -> Vec<String> {
    let mut file = File::open(Path::new(dir).join("input.txt")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
}
