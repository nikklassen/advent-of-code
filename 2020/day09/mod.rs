#![allow(clippy::needless_range_loop)]

use crate::utils;

lazy_static! {
    static ref INPUT: Vec<String> = utils::read_input_lines("day09");
}

fn read_nums() -> Vec<usize> {
    INPUT.iter().map(|s| s.parse().unwrap()).collect()
}

pub fn find_invalid(nums: &[usize]) -> usize {
    let windows = nums.array_windows::<26>();
    for window in windows {
        let n = window[25];
        if utils::sum2(&window[..25], n).is_none() {
            return n;
        }
    }
    unreachable!()
}

pub fn part1() -> usize {
    let nums = read_nums();
    find_invalid(&nums)
}

fn find_range(nums: &[usize], target_sum: usize) -> (usize, usize) {
    for window_len in 2..nums.len() {
        let mut tot = 0;
        for n in nums[0..window_len].iter() {
            tot += n;
        }
        if tot == target_sum {
            return (0, window_len);
        }
        for i in window_len..nums.len() {
            tot = tot - nums[i - window_len] + nums[i];
            if tot == target_sum {
                return (i - window_len + 1, i + 1);
            }
        }
    }
    unreachable!()
}

pub fn part2() -> usize {
    let nums = read_nums();
    let invalid = find_invalid(&nums);
    let (start, end) = find_range(&nums, invalid);
    let min = nums[start..end].iter().min().unwrap();
    let max = nums[start..end].iter().max().unwrap();
    min + max
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn run_part1() {
        assert_eq!(part1(), 177777905);
    }

    #[test]
    fn run_part2() {
        assert_eq!(part2(), 23463012);
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
