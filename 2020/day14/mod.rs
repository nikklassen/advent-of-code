use crate::utils::{self, *};

use regex::Regex;

lazy_static! {
    static ref INPUT: Vec<String> = utils::read_input_lines("day14");
    static ref MEM_RE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
}

fn read_input() -> Vec<Command> {
    INPUT
        .iter()
        .map(|line| {
            if let Some(mask_str) = line.strip_prefix("mask = ") {
                Command::UpdateMask(mask_str.parse::<Mask>().unwrap())
            } else {
                let captures = MEM_RE.captures(line).unwrap();
                Command::Write {
                    address: captures[1].to_string().parse().unwrap(),
                    value: captures[2].to_string().parse().unwrap(),
                }
            }
        })
        .collect()
}

#[derive(Copy, Clone)]
struct Mask {
    x_bits: u64,
    set_bits: u64,
}

fn mask_for_i(mask: u64, i: u64) -> u64 {
    let mut bit = 1;
    let mut next_mask = 0;
    let mut n_i = 1;
    while bit < (1 << 36) {
        if mask & bit > 0 {
            if i & n_i > 0 {
                next_mask |= bit;
            }
            n_i <<= 1;
        }
        bit <<= 1;
    }
    next_mask
}

impl Mask {
    fn apply(&self, other: u64) -> u64 {
        (other & self.x_bits) | self.set_bits
    }

    fn all_values(&self, mut address: u64) -> Vec<u64> {
        address |= self.set_bits;

        let mut bit = 1 << 36;
        let mut c = 1;
        while bit > 0 {
            if self.x_bits & bit > 0 {
                c *= 2;
            }
            bit >>= 1;
        }
        let mut values = vec![0; c];
        for (i, val) in values.iter_mut().enumerate() {
            let mask_i = mask_for_i(self.x_bits, i as u64);
            *val = mask_i | ((!self.x_bits) & address);
        }
        values
    }
}

impl std::fmt::Debug for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "allow: {:36b}, force: {:36b}",
            self.x_bits, self.set_bits
        )
    }
}

impl std::str::FromStr for Mask {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .fold(Some((0, 0)), |acc, c| {
                acc.and_then(|(mut x_bits, mut set_bits)| {
                    x_bits <<= 1;
                    set_bits <<= 1;
                    match c {
                        '0' => {}
                        '1' => {
                            set_bits |= 1;
                        }
                        'X' => {
                            x_bits |= 1;
                        }
                        _ => {
                            return None;
                        }
                    };
                    Some((x_bits, set_bits))
                })
            })
            .map(|(x_bits, set_bits)| Mask { x_bits, set_bits })
            .ok_or(())
    }
}

#[derive(Debug)]
enum Command {
    UpdateMask(Mask),
    Write { address: u64, value: u64 },
}

#[derive(Debug)]
struct State {
    mask: Mask,
    mem: HashMap<u64, u64>,
}

fn run_decoder<F>(run_command: F) -> u64
where
    F: Fn(State, &Command) -> State,
{
    let commands = read_input();
    let final_state = commands.iter().fold(
        State {
            mask: Mask {
                x_bits: 0,
                set_bits: 0,
            },
            mem: HashMap::new(),
        },
        run_command,
    );
    final_state.mem.values().sum()
}

pub fn part1() -> u64 {
    run_decoder(|mut state, command| {
        match command {
            Command::UpdateMask(mask) => {
                state.mask = *mask;
            }
            Command::Write { address, value } => {
                state.mem.insert(*address, state.mask.apply(*value));
            }
        };
        state
    })
}

pub fn part2() -> u64 {
    run_decoder(|mut state, command| {
        match command {
            Command::UpdateMask(mask) => {
                state.mask = *mask;
            }
            Command::Write { address, value } => {
                for i in state.mask.all_values(*address) {
                    state.mem.insert(i, *value);
                }
            }
        };
        state
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn run_part1() {
        assert_eq!(part1(), 14839536808842);
    }

    #[test]
    fn run_part2() {
        assert_eq!(part2(), 4215284199669);
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
