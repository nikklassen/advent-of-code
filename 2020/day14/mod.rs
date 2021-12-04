use std::lazy::OnceCell;

use shared::utils;

use ahash::AHashMap;

lazy_static! {
    static ref INPUT: Vec<String> = utils::read_input_lines("day14");
}

fn read_input() -> Vec<Command> {
    INPUT
        .iter()
        .map(|line| {
            if let Some(mask_str) = line.strip_prefix("mask = ") {
                Command::UpdateMask(mask_str.parse::<Mask>().unwrap())
            } else {
                // Format: ^mem\[\d+\] = \d+$
                let mut n = 0;
                let mut address = 0u64;
                for c in line[4..].chars() {
                    if c.is_digit(10) {
                        n = n * 10 + ((c as u64) - ('0' as u64));
                    } else if n > 0 {
                        address = n;
                        n = 0;
                    }
                }
                Command::Write { address, value: n }
            }
        })
        .collect()
}

struct Mask {
    x_bits: u64,
    set_bits: u64,
    all_values_cache: OnceCell<Vec<u64>>,
}

fn mask_for_i(mut mask: u64, mut i: u64) -> u64 {
    let mut next_mask = 0;
    let mut b_num = 0;
    while mask > 0 {
        let m = mask & 1;
        next_mask |= (i & m) << b_num;
        // Go to the next bit of i if the current mask bit was 1.
        i >>= m;
        mask >>= 1;
        b_num += 1;
    }
    next_mask
}

impl Mask {
    fn new(x_bits: u64, set_bits: u64) -> Self {
        Mask {
            x_bits,
            set_bits,
            all_values_cache: OnceCell::new(),
        }
    }

    fn apply(&self, other: u64) -> u64 {
        (other & self.x_bits) | self.set_bits
    }

    fn all_values(&self, mut address: u64) -> Vec<u64> {
        let masks = self.all_values_cache.get_or_init(|| {
            let c = 2usize.pow(self.x_bits.count_ones());
            let mut masks = vec![0; c];
            for i in 0..c {
                masks[i] = mask_for_i(self.x_bits, i as u64);
            }
            masks
        });

        address |= self.set_bits;
        let mut values = vec![0; masks.len()];
        for i in 0..masks.len() {
            values[i] = masks[i] | ((!self.x_bits) & address);
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
        let mut x_bits = 0;
        let mut set_bits = 0;
        for c in s.chars() {
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
                    return Err(());
                }
            }
        }
        Ok(Mask::new(x_bits, set_bits))
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
    mem: AHashMap<u64, u64>,
}

fn run_decoder<F>(run_command: F) -> u64
where
    F: Fn(&mut State, Command),
{
    let commands = read_input();
    let mut state = State {
        mask: Mask::new(0, 0),
        mem: AHashMap::new(),
    };
    for cmd in commands.into_iter() {
        run_command(&mut state, cmd);
    }
    state.mem.values().sum()
}

pub fn part1() -> u64 {
    run_decoder(|mut state, command| {
        match command {
            Command::UpdateMask(mask) => {
                state.mask = mask;
            }
            Command::Write { address, value } => {
                state.mem.insert(address, state.mask.apply(value));
            }
        };
    })
}

pub fn part2() -> u64 {
    run_decoder(|mut state, command| {
        match command {
            Command::UpdateMask(mask) => {
                state.mask = mask;
            }
            Command::Write { address, value } => {
                for i in state.mask.all_values(address) {
                    state.mem.insert(i, value);
                }
            }
        };
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
