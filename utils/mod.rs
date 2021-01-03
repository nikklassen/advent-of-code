pub use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use itertools::Itertools;

pub fn read_input_lines(dir: &str) -> Vec<String> {
    let mut file = File::open(Path::new(dir).join("input.txt")).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
}

pub fn parse_input_nums<F>(dir: &str) -> Vec<F>
where
    F: std::str::FromStr,
    F::Err: std::fmt::Debug,
{
    let input = read_input_lines(dir);
    input.iter().map(|s| s.parse().unwrap()).collect()
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

pub type GridIndex = (usize, usize);

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct GridDir(isize, isize);

pub static ADJACENT_DIRS: [GridDir; 8] = [
    GridDir(-1, -1),
    GridDir(-1, 0),
    GridDir(-1, 1),
    GridDir(0, -1),
    GridDir(0, 1),
    GridDir(1, -1),
    GridDir(1, 0),
    GridDir(1, 1),
];

pub fn increment_pos<T>(grid: &[Vec<T>], pos: GridIndex, dir: GridDir) -> Option<GridIndex> {
    if pos.0 == 0 && dir.0 < 0
        || pos.0 == grid.len() - 1 && dir.0 > 0
        || pos.1 == 0 && dir.1 < 0
        || pos.1 == grid[pos.0].len() - 1 && dir.1 > 0
    {
        None
    } else {
        Some((
            ((pos.0 as isize) + dir.0) as usize,
            ((pos.1 as isize) + dir.1) as usize,
        ))
    }
}

pub fn pos_index<T>(grid: &[Vec<T>], pos: GridIndex) -> &T {
    &grid[pos.0][pos.1]
}
