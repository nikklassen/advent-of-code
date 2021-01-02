use std::collections::{HashMap, HashSet};

use crate::utils;

fn read_nums() -> Vec<usize> {
    let input = utils::read_input_lines("day09");
    input.iter().map(|s| s.parse().unwrap()).collect()
}

pub fn find_invalid(nums: &[usize]) -> usize {
    let mut sums = [[0; 25]; 25];
    let mut sum_locs = HashMap::<usize, HashSet<(usize, usize)>>::new();
    for i in 0..25 {
        for j in 0..25 {
            if i == j {
                continue;
            }
            let sum = nums[i] + nums[j];
            sums[i][j] = sum;
            let locs = sum_locs.entry(sum).or_default();
            locs.insert((i, j));
        }
    }
    for k in 25..nums.len() {
        let current_sum = nums[k];
        if !sum_locs.contains_key(&current_sum) {
            return current_sum;
        }
        let prev = nums[k - 25];
        let j = k % 25;
        for i in 0..25 {
            if i == j {
                continue;
            }
            sum_locs.get_mut(&sums[i][j]).unwrap().remove(&(i, j));
            sums[i][j] = sums[i][j] - prev + current_sum;
            sum_locs.entry(sums[i][j]).or_default().insert((i, j));

            sum_locs.get_mut(&sums[j][i]).unwrap().remove(&(j, i));
            sums[j][i] = sums[j][i] - prev + current_sum;
            sum_locs.entry(sums[j][i]).or_default().insert((j, i));
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
