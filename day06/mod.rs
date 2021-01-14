use std::collections::HashSet;

use crate::utils;

lazy_static! {
    static ref INPUT: Vec<String> = utils::read_input_lines("day06");
}

fn read_answers() -> Vec<Vec<&'static String>> {
    utils::group_lines(&INPUT)
}

pub fn part1() -> usize {
    read_answers()
        .iter()
        .map(|group| {
            let set = group
                .iter()
                .flat_map(|answers| answers.chars())
                .collect::<HashSet<char>>();
            set.len()
        })
        .sum()
}

pub fn part2() -> usize {
    read_answers()
        .iter()
        .map(|group| {
            let sets = group
                .iter()
                .map(|answers| answers.chars().collect::<HashSet<char>>())
                .collect::<Vec<_>>();
            sets[1..]
                .iter()
                .fold(sets[0].clone(), |mut acc, next| {
                    acc.retain(|i| next.contains(i));
                    acc
                })
                .len()
        })
        .sum()
}
