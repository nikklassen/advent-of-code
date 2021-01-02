use std::collections::{HashMap, HashSet};

use crate::utils;

fn read_nums() -> Vec<usize> {
    let input = utils::read_input_lines("day09");
    input.iter().map(|s| s.parse().unwrap()).collect()
}

pub fn find_invalid(nums: &[usize]) -> usize {
    let mut sum_locs = HashMap::<usize, HashSet<(usize, usize)>>::new();
    for i in 0..25 {
        for j in (i + 1)..25 {
            sum_locs
                .entry(nums[i] + nums[j])
                .or_default()
                .insert((i, j));
        }
    }
    for end in 25..nums.len() {
        let target = nums[end];
        if !sum_locs.contains_key(&target) {
            return target;
        }
        let j = end % 25;
        for locs in sum_locs.values_mut() {
            locs.retain(|loc| loc.0 != j && loc.1 != j);
        }
        sum_locs.retain(|_, locs| !locs.is_empty());

        for cursor in 0..25 {
            let i = end - 25 + cursor;
            if i == j {
                continue;
            }
            sum_locs
                .entry(nums[i] + nums[end])
                .or_default()
                .insert((i, end));
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
