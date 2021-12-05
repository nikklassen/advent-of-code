pub use ahash::{AHashMap, AHashSet};
pub use std::collections::{HashMap, HashSet};
use std::default::Default;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use itertools::Itertools;

fn read_file_lines(dir: &str, file_name: &str) -> Vec<String> {
    let mut file = File::open(Path::new(dir).join(file_name)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
}

pub fn read_input_lines(dir: &str) -> Vec<String> {
    read_file_lines(dir, "input.txt")
}

pub fn read_sample_input_lines(dir: &str) -> Vec<String> {
    read_file_lines(dir, "sample_input.txt")
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

pub fn rotate_grid<T: Clone + Default>(grid: &[Vec<T>], theta: f32) -> Vec<Vec<T>> {
    let bound = (grid.len() - 1) as f32 / 2.0;
    let (sin_t, cos_t) = theta.sin_cos();

    let mut new_grid = vec![];
    for row in grid.iter() {
        new_grid.push(vec![T::default(); row.len()]);
    }
    for y in 0..grid.len() {
        let rel_y = (y as f32) - bound;
        for x in 0..grid.len() {
            let rel_x = (x as f32) - bound;

            let new_x = ((rel_x as f32) * cos_t - (rel_y as f32) * sin_t) + bound;
            let new_y = ((rel_x as f32) * sin_t + (rel_y as f32) * cos_t) + bound;

            new_grid[new_y.round() as usize][new_x.round() as usize] = grid[y][x].clone();
        }
    }

    new_grid
}

pub fn flip_grid_mut<T>(grid: &mut Vec<Vec<T>>, x: bool, y: bool) {
    if x {
        for row in grid.iter_mut() {
            row.reverse();
        }
    }
    if y {
        grid.reverse();
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_flip_x() {
        let mut g = vec![vec![1, 0, 1], vec![0, 1, 0], vec![0, 0, 1]];
        flip_grid_mut(&mut g, true, false);
        let want_g = vec![vec![0, 0, 1], vec![0, 1, 0], vec![1, 0, 1]];
        assert_eq!(g, want_g);
    }

    #[test]
    fn graph_flip_y() {
        let mut g = vec![vec![1, 0, 1], vec![0, 1, 0], vec![0, 0, 1]];
        flip_grid_mut(&mut g, false, true);
        let want_g = vec![vec![1, 0, 1], vec![0, 1, 0], vec![1, 0, 0]];
        assert_eq!(g, want_g);
    }
}
