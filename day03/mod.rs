use crate::utils;

use std::iter;

fn read_map() -> Vec<Vec<bool>> {
    let input = utils::read_input_lines("day03");
    input
        .iter()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn slide(m: &Vec<Vec<bool>>, slope: (usize, usize)) -> usize {
    let height = m.len();
    let width = m[0].len();
    iter::successors(Some((0, 0)), move |pos| {
        let y = pos.1 + slope.1;
        if y < height {
            Some((pos.0 + slope.0, y))
        } else {
            None
        }
    })
    .map(move |pos| m[pos.1][pos.0 % width])
    .fold(0, |acc, v| if v { acc + 1 } else { acc })
}

pub fn part1() -> usize {
    let m = read_map();
    slide(&m, (3, 1))
}

pub fn part2() -> usize {
    let m = read_map();
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes
        .iter()
        .map(|&slope| slide(&m, slope))
        .fold(1, |acc, v| acc * v)
}
