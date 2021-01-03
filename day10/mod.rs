use crate::utils;

fn read_adapters() -> Vec<i32> {
    utils::parse_input_nums("day10")
}

type Count = (i32, i32, i32);

fn add_to_count(c: &Count, diff: i32) -> Count {
    match diff {
        1 => (c.0 + 1, c.1, c.2),
        2 => (c.0, c.1 + 1, c.2),
        3 => (c.0, c.1, c.2 + 1),
        _ => panic!("invalid diff: {}", diff),
    }
}

pub fn part1() -> i32 {
    let mut adapters = read_adapters();
    adapters.sort_unstable();

    let mut ones = 0;
    let mut threes = 1;
    let mut last_joltage = 0;
    for adapter in adapters {
        if adapter - 1 == last_joltage {
            ones += 1;
        } else if adapter - 3 == last_joltage {
            threes += 1;
        }
        last_joltage = adapter;
    }
    ones * threes
}

fn count_chains(mut adapters: Vec<i32>) -> usize {
    adapters.insert(0, 0);
    adapters.push(adapters[adapters.len() - 1] + 3);

    let mut memo = vec![0; adapters.len()];
    memo[0] = 1;
    for i in 0..adapters.len() {
        for j in (i + 1)..adapters.len() {
            if adapters[i] + 3 < adapters[j] {
                break;
            }
            memo[j] += memo[i];
        }
    }
    memo[adapters.len() - 1]
}

pub fn part2() -> usize {
    let mut adapters = read_adapters();
    adapters.sort_unstable();

    count_chains(adapters)
}
