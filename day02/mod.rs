use regex::Regex;

use crate::utils;

struct PwdEntry {
    v1: usize,
    v2: usize,
    c: char,
    pwd: String,
}

fn read_entries() -> Vec<PwdEntry> {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let input = utils::read_input_lines("day02/input");
    input
        .iter()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            PwdEntry {
                v1: captures[1].parse().unwrap(),
                v2: captures[2].parse().unwrap(),
                c: captures[3].chars().nth(0).unwrap(),
                pwd: captures[4].to_string(),
            }
        })
        .collect()
}

fn is_policy_match1(e: &PwdEntry) -> bool {
    let count = e.pwd.chars().filter(|&c| c == e.c).count();
    count >= e.v1 && count <= e.v2
}

pub fn part1() -> usize {
    let entries = read_entries();
    entries.iter().filter(|e| is_policy_match1(e)).count()
}

fn is_policy_match2(e: &PwdEntry) -> bool {
    let chars = e.pwd.chars().collect::<Vec<_>>();
    (chars[e.v1 - 1] == e.c) ^ (chars[e.v2 - 1] == e.c)
}

pub fn part2() -> usize {
    let entries = read_entries();
    entries.iter().filter(|e| is_policy_match2(e)).count()
}
