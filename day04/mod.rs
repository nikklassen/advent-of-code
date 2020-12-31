use std::collections::HashMap;
use std::iter::Iterator;

use itertools::Itertools;
use regex::Regex;

use crate::utils;

fn read_passports() -> Vec<HashMap<String, String>> {
    let input = utils::read_input_lines("day04");
    input
        .iter()
        .group_by(|line| line.as_str() != "")
        .into_iter()
        .filter_map(|(v, lines)| {
            if !v {
                return None;
            }
            Some(
                lines
                    .map(|line| {
                        line.split(' ').map(|kvp| {
                            let parts = kvp.split(':').collect::<Vec<_>>();
                            (parts[0].to_string(), parts[1].to_string())
                        })
                    })
                    .flatten()
                    .collect(),
            )
        })
        .collect()
}

pub fn part1() -> usize {
    let passes = read_passports();
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    passes
        .iter()
        .filter(|pass| {
            required_fields
                .iter()
                .all(|key| pass.contains_key(&key.to_string()))
        })
        .count()
}

pub fn part2() -> usize {
    let passes = read_passports();
    let mut validation_regexes = HashMap::new();
    validation_regexes.insert(
        "byr".to_string(),
        Regex::new(r"^(19[2-9][0-9]|200[0-2])$").unwrap(),
    );
    validation_regexes.insert("iyr".to_string(), Regex::new(r"^20(1[0-9]|20)$").unwrap());
    validation_regexes.insert("eyr".to_string(), Regex::new(r"^20(2[0-9]|30)$").unwrap());
    validation_regexes.insert(
        "hgt".to_string(),
        Regex::new(r"^(1([5-8][0-9]|9[0-3])cm|(59|[6-8][0-9]|9[0-3])in)$").unwrap(),
    );
    validation_regexes.insert("hcl".to_string(), Regex::new(r"^#[0-9a-f]{6}$").unwrap());
    validation_regexes.insert(
        "ecl".to_string(),
        Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap(),
    );
    validation_regexes.insert("pid".to_string(), Regex::new(r"^[0-9]{9}$").unwrap());

    passes
        .iter()
        .filter(|pass| {
            validation_regexes.iter().all(|(key, validation_regex)| {
                if let Some(value) = pass.get(key) {
                    validation_regex.is_match(value)
                } else {
                    false
                }
            })
        })
        .count()
}
