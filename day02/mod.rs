use crate::utils;

struct PwdEntry<'a> {
    v1: usize,
    v2: usize,
    c: u8,
    pwd: &'a [u8],
}

fn read_entries(input: &[String]) -> Vec<PwdEntry> {
    let mut entries = Vec::with_capacity(input.len());
    for line in input {
        let dash_idx = line.find('-').unwrap();
        let v1 = line[..dash_idx].parse().unwrap();

        let space_idx = line[(dash_idx + 1)..].find(' ').unwrap() + dash_idx + 1;
        let v2 = line[(dash_idx + 1)..space_idx].parse().unwrap();

        let c = line.as_bytes()[space_idx + 1];

        let pwd = &line.as_bytes()[(space_idx + 4)..];

        entries.push(PwdEntry { v1, v2, c, pwd })
    }
    entries
}

fn is_policy_match1(e: &PwdEntry) -> bool {
    let mut count = 0;
    for b in e.pwd {
        if *b == e.c {
            count += 1;
        }
        if count > e.v2 {
            return false;
        }
    }
    count >= e.v1
}

pub fn part1(input: &[String]) -> usize {
    let entries = read_entries(input);
    utils::fast_count(&entries, is_policy_match1)
}

fn is_policy_match2(e: &PwdEntry) -> bool {
    (e.pwd[e.v1 - 1] == e.c) ^ (e.pwd[e.v2 - 1] == e.c)
}

pub fn part2(input: &[String]) -> usize {
    let entries = read_entries(input);
    utils::fast_count(&entries, is_policy_match2)
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::Bencher;

    lazy_static! {
        static ref INPUT: Vec<String> = utils::read_input_lines("day02");
    }

    #[test]
    fn run_part1() {
        assert_eq!(part1(&INPUT), 469);
    }

    #[test]
    fn run_part2() {
        assert_eq!(part2(&INPUT), 267);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(|| part1(&INPUT));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(|| part2(&INPUT));
    }
}
