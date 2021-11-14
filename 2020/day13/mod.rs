use crate::utils;
use std::iter::*;

lazy_static! {
    static ref INPUT: Vec<String> = utils::read_input_lines("day13");
}

fn read_schedules() -> (usize, Vec<Option<usize>>) {
    let earliest: usize = INPUT[0].parse().unwrap();
    let busses = INPUT[1]
        .split(',')
        .map(|b| b.parse::<usize>().ok())
        .collect();
    (earliest, busses)
}

pub fn part1() -> usize {
    let (start, busses) = read_schedules();
    let running_busses = busses.iter().filter_map(|b| *b).collect::<Vec<_>>();

    let mut t = start;
    loop {
        for bus in running_busses.iter() {
            if t % bus == 0 {
                return (t - start) * bus;
            }
        }
        t += 1;
    }
}

fn find_next(start: usize, prev_lcm: usize, i: usize, bus_i: usize) -> usize {
    let mut t = start;
    loop {
        let rem = (t + i) % bus_i;
        if rem != 0 {
            t += prev_lcm;
            continue;
        }
        return t;
    }
}

pub fn part2() -> usize {
    let (_, busses) = read_schedules();
    let indexed_busses = busses
        .iter()
        .enumerate()
        .filter_map(|(i, b)| b.map(|bv| (i, bv)))
        .collect::<Vec<_>>();

    let mut t = 0;
    for i in 1..indexed_busses.len() {
        let mut l = 1;
        for (_, bus_j) in indexed_busses[..i].iter() {
            l = utils::lcm(l, *bus_j);
        }

        let (bus_index, bus) = indexed_busses[i];
        t = find_next(t, l, bus_index, bus);
    }
    t
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn run_part1() {
        assert_eq!(part1(), 102);
    }

    #[test]
    fn run_part2() {
        assert_eq!(part2(), 327300950120029);
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
