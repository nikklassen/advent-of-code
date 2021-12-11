use shared::utils;

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day06");
    static ref INPUT: Vec<String> = utils::read_input_lines("day06");
}

fn parse_input() -> Vec<usize> {
    let line = &INPUT[0];
    line.split(',').map(|part| part.parse().unwrap()).collect()
}

fn run(final_day: usize) -> usize {
    let mut fish1 = 0;
    let mut fish2 = 0;

    let mut fish_per_day = [0; 7];
    for fish in parse_input().iter() {
        fish_per_day[*fish] += 1;
    }

    for day in 1.. {
        let new_fish = fish1;
        fish1 = fish2;

        fish2 = fish_per_day[(day - 1) % 7];

        fish_per_day[(day - 1) % 7] += new_fish;

        if day == final_day {
            break;
        }
    }
    fish_per_day.iter().sum::<usize>() + fish1 + fish2
}

pub fn part1() -> usize {
    run(80)
}

pub fn part2() -> usize {
    run(256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_part1() {
        assert_eq!(part1(), 390923);
    }

    #[test]
    fn run_part2() {
        assert_eq!(part2(), 1749945484935)
    }
}
