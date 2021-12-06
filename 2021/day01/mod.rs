use shared::utils;

use itertools::Itertools;

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day01");
    static ref INPUT: Vec<String> = utils::read_input_lines("day01");
}

fn parse_input() -> Vec<usize> {
    utils::parse_nums(&INPUT)
}

fn count_increases<'a>(i: &[usize]) -> usize {
    i.iter()
        .fold((0, None), |(c, prev_opt), v| {
            if let Some(prev) = prev_opt {
                (if v > prev { c + 1 } else { c }, Some(v))
            } else {
                (0, Some(v))
            }
        })
        .0
}

pub fn part1() -> usize {
    count_increases(&parse_input()[..])
}

pub fn part2() -> usize {
    let nums = parse_input();
    count_increases(&nums.windows(3).map(|w| w.iter().sum()).collect_vec()[..])
}
