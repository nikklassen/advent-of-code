use crate::utils;

use std::iter;

lazy_static! {
    static ref INPUT: Vec<String> = utils::read_input_lines("day03");
}

fn read_map() -> Vec<Vec<bool>> {
    let mut map = vec![vec![false; INPUT[0].len()]; INPUT.len()];
    for i in 0..INPUT.len() {
        let row = &INPUT[i];
        for j in 0..row.len() {
            if INPUT[i].as_bytes()[j] == b'#' {
                map[i][j] = true;
            }
        }
    }
    map
}

fn slide(m: &[Vec<bool>], slope: (usize, usize)) -> usize {
    let height = m.len();
    let width = m[0].len();
    // Surprisingly this is basically the same performance as the loop-based
    // version.
    iter::successors(Some((0, 0)), move |pos| {
        let y = pos.1 + slope.1;
        if y < height {
            Some((pos.0 + slope.0, y))
        } else {
            None
        }
    })
    .map(move |pos| if m[pos.1][pos.0 % width] { 1 } else { 0 })
    .sum()
}

pub fn part1() -> usize {
    let m = read_map();
    slide(&m, (3, 1))
}

pub fn part2() -> usize {
    let m = read_map();
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes.iter().map(|&slope| slide(&m, slope)).product()
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::Bencher;

    #[test]
    fn run_part1() {
        assert_eq!(part1(), 191);
    }

    #[test]
    fn run_part2() {
        assert_eq!(part2(), 1478615040);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part2);
    }
}
