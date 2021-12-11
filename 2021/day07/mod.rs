use rdxsort::*;
use shared::utils;

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day07");
    static ref INPUT: Vec<String> = utils::read_input_lines("day07");
}

fn parse_input() -> Vec<u16> {
    INPUT[0]
        .split(',')
        .map(|part| part.parse().unwrap())
        .collect()
}

fn run<F>(fuel_calc: F) -> usize
where
    F: Fn(isize, isize) -> isize,
{
    let crabs = parse_input();
    let max = crabs.iter().max().unwrap();
    (0..*max)
        .map(|p| {
            let mut tot = 0;
            for c in crabs.iter() {
                tot += fuel_calc(*c as isize, p as isize) as usize;
            }
            tot
        })
        .min()
        .unwrap()
}

pub fn part1_impl1() -> isize {
    run(|crab_pos, p| (crab_pos - p).abs()) as isize
}

pub fn part1_impl2() -> isize {
    let mut crabs = parse_input();
    crabs.rdxsort();

    let mut max = 0;
    let mut curr_fuel = 0isize;
    for &c in crabs.iter() {
        if c > max {
            max = c
        }
        curr_fuel += c as isize;
    }

    let mut num_bigger = crabs.len() as isize;
    let mut num_smaller: isize = 0;
    let mut best = isize::MAX;

    let mut next_crab_idx = 0;
    for p in 0..max {
        while p == crabs[next_crab_idx] {
            next_crab_idx += 1;
            num_bigger -= 1;
            num_smaller += 1;
        }
        curr_fuel = curr_fuel - num_bigger + num_smaller;
        if curr_fuel < best {
            best = curr_fuel;
        }
    }
    best
}

pub fn part1() -> isize {
    part1_impl2()
}

pub fn part2() -> usize {
    run(|crab_pos, p| {
        let dist = (crab_pos - p).abs();
        (dist * (dist + 1)) / 2
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn run_part1_impl1() {
        assert_eq!(part1_impl1(), 335330);
    }

    #[test]
    fn run_part1_impl2() {
        assert_eq!(part1_impl2(), 335330);
    }

    #[test]
    fn run_part2_impl1() {
        assert_eq!(part2(), 92439766);
    }

    #[bench]
    fn bench_part1_impl1(b: &mut Bencher) {
        b.iter(part1_impl1);
    }

    #[bench]
    fn bench_part1_impl2(b: &mut Bencher) {
        b.iter(part1_impl2);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
