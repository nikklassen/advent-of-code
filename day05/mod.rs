use crate::utils;

fn read_seat_ids() -> Vec<i32> {
    let input = utils::read_input_lines("day05");
    input.iter().map(|line| parse_seat_id(line)).collect()
}

fn parse_seat_id(s: &str) -> i32 {
    let mut digit = 1;
    let mut tot = 0;
    for c in s.chars().rev() {
        if c == 'B' || c == 'R' {
            tot += digit;
        }
        digit *= 2;
    }
    tot
}

pub fn part1() -> i32 {
    *read_seat_ids().iter().max().unwrap()
}

pub fn part2() -> i32 {
    let mut seat_ids = read_seat_ids();
    seat_ids.sort();
    for i in 1..(seat_ids.len()) {
        if seat_ids[i] != seat_ids[i - 1] + 1 {
            assert!(seat_ids[i] == seat_ids[i - 1] + 2);
            return seat_ids[i] - 1;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seat() {
        struct TestCase {
            input: &'static str,
            seat_id: i32,
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
}
