use pcre2::bytes::Regex;

use crate::utils::{self, *};

lazy_static! {
    static ref INPUT: Vec<String> = utils::read_input_lines("day04");
}

fn read_passports() -> Vec<HashMap<&'static str, &'static str>> {
    let mut passports = Vec::with_capacity(INPUT.len());
    passports.push(HashMap::with_capacity(8));
    let mut passport = passports.last_mut().unwrap();
    for line in INPUT.iter() {
        if line.is_empty() {
            passports.push(HashMap::with_capacity(8));
            passport = passports.last_mut().unwrap();
            continue;
        }

        for kvp in line.split(' ') {
            passport.insert(&kvp[..3], &kvp[4..]);
        }
    }
    passports
}

pub fn part1() -> usize {
    let passes = read_passports();
    passes
        .iter()
        .filter(|pass| pass.len() == 7 && !pass.contains_key("cid") || pass.len() == 8)
        .count()
}

pub fn part2() -> usize {
    let passes = read_passports();
    let mut validation_regexes: HashMap<&'static str, Regex> = HashMap::with_capacity(7);
    validation_regexes.insert("byr", Regex::new(r"^(?:19[2-9][0-9]|200[0-2])$").unwrap());
    validation_regexes.insert("iyr", Regex::new(r"^20(?:1[0-9]|20)$").unwrap());
    validation_regexes.insert("eyr", Regex::new(r"^20(?:2[0-9]|30)$").unwrap());
    validation_regexes.insert(
        "hgt",
        Regex::new(r"^(?:1(?:[5-8][0-9]|9[0-3])cm|(?:59|[6-8][0-9]|9[0-3])in)$").unwrap(),
    );
    validation_regexes.insert("hcl", Regex::new(r"^#[0-9a-f]{6}$").unwrap());
    validation_regexes.insert(
        "ecl",
        Regex::new(r"^(?:amb|blu|brn|gry|grn|hzl|oth)$").unwrap(),
    );
    validation_regexes.insert("pid", Regex::new(r"^[0-9]{9}$").unwrap());

    passes
        .iter()
        .filter(|pass| {
            (pass.len() == 7 && !pass.contains_key("cid") || pass.len() == 8)
                && validation_regexes.iter().all(|(key, validation_regex)| {
                    if let Some(value) = pass.get(key) {
                        validation_regex.is_match(value.as_bytes()).unwrap()
                    } else {
                        false
                    }
                })
        })
        .count()
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::Bencher;

    #[test]
    fn run_part1() {
        assert_eq!(part1(), 228);
    }

    #[test]
    fn run_part2() {
        assert_eq!(part2(), 175);
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
