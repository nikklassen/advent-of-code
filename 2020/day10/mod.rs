use shared::utils;

lazy_static! {
    static ref INPUT: Vec<String> = utils::read_input_lines("day10");
}

fn read_adapters() -> Vec<i32> {
    utils::parse_nums(&INPUT)
}

pub fn part1() -> i32 {
    let mut adapters = read_adapters();
    adapters.sort_unstable();

    let mut ones = 0;
    let mut threes = 1;
    let mut last_joltage = 0;
    for adapter in adapters {
        if adapter - 1 == last_joltage {
            ones += 1;
        } else if adapter - 3 == last_joltage {
            threes += 1;
        }
        last_joltage = adapter;
    }
    ones * threes
}

fn count_chains(mut adapters: Vec<i32>) -> usize {
    adapters.insert(0, 0);
    adapters.push(adapters[adapters.len() - 1] + 3);

    let mut memo = vec![0; adapters.len()];
    memo[0] = 1;
    for i in 0..adapters.len() {
        for j in (i + 1)..adapters.len() {
            if adapters[i] + 3 < adapters[j] {
                break;
            }
            memo[j] += memo[i];
        }
    }
    memo[adapters.len() - 1]
}

pub fn part2() -> usize {
    let mut adapters = read_adapters();
    adapters.sort_unstable();

    count_chains(adapters)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn run_part1() {
        assert_eq!(part1(), 2482);
    }

    #[test]
    fn run_part2() {
        assert_eq!(part2(), 96717311574016);
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
