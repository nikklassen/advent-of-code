#![allow(clippy::needless_range_loop)]

use shared::utils;

lazy_static! {
    static ref INPUT: Vec<String> = utils::read_input_lines("day05");
}

fn read_seat_ids() -> impl std::iter::Iterator<Item = usize> {
    INPUT.iter().map(|line| parse_seat_id(line))
}

fn parse_seat_id(s: &str) -> usize {
    let mut digit = 1;
    let mut tot = 0;
    for &c in s.as_bytes().iter().rev() {
        if c == b'B' || c == b'R' {
            tot += digit;
        }
        digit *= 2;
    }
    tot
}

pub fn part1() -> usize {
    read_seat_ids().max().unwrap()
}

pub fn part2() -> usize {
    let mut seat_ids = read_seat_ids().collect::<Vec<_>>();
    seat_ids.sort_unstable();
    for i in 1..(seat_ids.len()) {
        if seat_ids[i] != seat_ids[i - 1] + 1 {
            return seat_ids[i] - 1;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_parse_seat() {
        struct TestCase {
            input: &'static str,
            seat_id: usize,
        }
        let tests = [
            TestCase {
                input: "BFFFBBFRRR",
                seat_id: 567,
            },
            TestCase {
                input: "FFFBBBFRRR",
                seat_id: 119,
            },
            TestCase {
                input: "BBFFBBFRLL",
                seat_id: 820,
            },
        ];
        for test in tests.iter() {
            let sid = parse_seat_id(test.input);
            assert_eq!(sid, test.seat_id, "{}", test.input);
        }
    }

    #[test]
    fn run_part1() {
        assert_eq!(part1(), 965);
    }

    #[test]
    fn run_part2() {
        assert_eq!(part2(), 524);
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
