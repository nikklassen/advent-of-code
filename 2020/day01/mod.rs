#![allow(clippy::needless_range_loop)]

use crate::utils;

pub fn part1(input: &[String]) -> usize {
    let mut vals = utils::parse_nums(input);
    let (vi, vj) = utils::sum2_mut(&mut vals, 2020).unwrap();
    vi * vj
}

pub fn part2(input: &[String]) -> usize {
    let mut vals: Vec<usize> = utils::parse_nums(input);
    vals.sort_unstable();

    let n = vals.len();
    let target = 2020;
    for k in 0..n {
        let vk = vals[k];
        for j in (k + 1)..n {
            let vjk = vk + vals[j];
            if vjk >= target {
                break;
            }
            for i in (j + 1)..n {
                let tot = vjk + vals[i];
                if tot == target {
                    return vk * vals[j] * vals[i];
                }
                if tot > target {
                    break;
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    lazy_static! {
        static ref INPUT: Vec<String> = utils::read_input_lines("day01");
    }

    use super::*;
    use test::Bencher;

    #[test]
    fn run_part1() {
        assert_eq!(part1(&INPUT), 157059);
    }

    #[test]
    fn run_part2() {
        assert_eq!(part2(&INPUT), 165080960);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| part1(&INPUT));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| part2(&INPUT));
    }
}
