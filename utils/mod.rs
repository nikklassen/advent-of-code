use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use itertools::Itertools;

pub fn read_input_lines(dir: &str) -> Vec<String> {
    let mut file = File::open(Path::new(dir).join("input.txt")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
}

pub fn group_lines(lines: Vec<String>) -> Vec<Vec<String>> {
    lines
        .into_iter()
        .group_by(|line| line.as_str() != "")
        .into_iter()
        .filter_map(|(v, lines)| {
            if !v {
                return None;
            }
            Some(lines.collect())
        })
        .collect()
}
