use crate::utils::{self, *};

fn read_input() -> Vec<usize> {
    let lines = utils::read_input_lines("day15");
    lines[0].split(',').map(|n| n.parse().unwrap()).collect()
}

fn nth(starting_nums: &[usize], mut n: usize) -> usize {
    let mut nums: HashMap<usize, usize> = HashMap::new();
    let mut t = starting_nums.len() - 1;
    let mut last = starting_nums[t];
    for (i, num) in starting_nums[..t].iter().enumerate() {
        nums.insert(*num, i);
    }
    loop {
        let next = if let Some(&j) = nums.get(&last) {
            t - j
        } else {
            0
        };
        nums.insert(last, t);
        t += 1;
        last = next;
        if n == 0 {
            return next;
        }
        n -= 1;
    }
}

pub fn part1() -> usize {
    let starting_nums = read_input();
    nth(&starting_nums, 2020 - starting_nums.len() - 1)
}

pub fn part2() -> usize {
    let starting_nums = read_input();
    nth(&starting_nums, 30000000 - starting_nums.len() - 1)
}
