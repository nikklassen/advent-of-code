use crate::utils;

lazy_static! {
    static ref INPUT: Vec<String> = utils::read_input_lines("day06");
}

fn read_answers() -> Vec<Vec<&'static String>> {
    utils::group_lines(&INPUT)
}

struct CharSet {
    vals: u32,
}

impl CharSet {
    pub fn new() -> Self {
        Self { vals: 0 }
    }

    pub fn full() -> Self {
        Self {
            vals: (1 << 27) - 1,
        }
    }

    pub fn contains(&self, c: u8) -> bool {
        self.vals & (1 << (c - b'a')) > 0
    }

    pub fn insert(&mut self, c: u8) {
        self.vals |= 1 << (c - b'a');
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            vals: self.vals & other.vals,
        }
    }

    pub fn len(&self) -> usize {
        let mut tot = 0;
        let mut b = self.vals;
        while b > 0 {
            tot += b & 1;
            b >>= 1;
        }
        tot as usize
    }
}

pub fn part1() -> usize {
    let mut tot = 0;
    for group in read_answers().iter() {
        let mut charset = CharSet::new();
        for line in group.iter() {
            for &c in line.as_bytes().iter() {
                if !charset.contains(c) {
                    tot += 1;
                    charset.insert(c);
                }
            }
        }
    }
    tot
}

pub fn part2() -> usize {
    let mut tot = 0;
    for group in read_answers().iter() {
        let mut charset = CharSet::full();
        for line in group.iter() {
            let mut charset2 = CharSet::new();
            for &c in line.as_bytes().iter() {
                charset2.insert(c);
            }
            charset = charset.union(&charset2);
        }
        tot += charset.len()
    }
    tot
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn run_part1() {
        assert_eq!(part1(), 6310);
    }

    #[test]
    fn run_part2() {
        assert_eq!(part2(), 3193);
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
