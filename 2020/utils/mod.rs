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
    parse_nums(&read_input_lines(dir))
}

pub fn parse_nums<F>(input: &[String]) -> Vec<F>
where
    F: std::str::FromStr,
    F::Err: std::fmt::Debug,
{
    input.iter().map(|s| s.parse().unwrap()).collect()
}

pub fn group_lines(lines: &[String]) -> Vec<Vec<&String>> {
    lines
        .iter()
        .group_by(|line| !line.is_empty())
        .into_iter()
        .filter_map(|(v, lines)| {
            if !v {
                return None;
            }
            Some(lines.into_iter().collect())
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

pub fn gcd(mut a: usize, mut b: usize) -> usize {
    loop {
        if b == 0 {
            return a;
        }
        let tmp = b;
        b = a % b;
        a = tmp;
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    (a / gcd(a, b)) * b
}

#[inline]
pub fn fast_count<T, F: Fn(&T) -> bool>(s: &[T], f: F) -> usize {
    let mut c = 0;
    for e in s {
        if f(e) {
            c += 1;
        }
    }
    c
}

pub fn copy_slice_to_vec<T: Copy>(s: &[T]) -> Vec<T> {
    let mut new = Vec::with_capacity(s.len());
    unsafe {
        new.set_len(s.len());
    }
    new.copy_from_slice(s);
    new
}

pub fn sum2(vs: &[usize], target: usize) -> Option<(usize, usize)> {
    let mut new = copy_slice_to_vec(vs);
    sum2_mut(&mut new, target)
}

pub fn sum2_mut(vs: &mut [usize], target: usize) -> Option<(usize, usize)> {
    vs.sort_unstable();

    let n = vs.len();
    for i in 0..n {
        let vi = vs[i];

        #[allow(clippy::needless_range_loop)]
        for j in (i + 1)..n {
            let vj = vs[j];
            if vi + vj == target {
                return Some((vi, vj));
            }
        }
    }
    None
}
