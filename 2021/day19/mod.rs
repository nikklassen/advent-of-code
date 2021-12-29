use std::fmt;

use ahash::AHashSet;
use shared::utils::{self, NextParser};

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day19");
    static ref INPUT: Vec<String> = utils::read_input_lines("day19");
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Hash, Default)]
struct Beacon {
    x: isize,
    y: isize,
    z: isize,
}

impl fmt::Debug for Beacon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, Default)]
struct Scanner {
    beacons: AHashSet<Beacon>,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Hash, Debug, Default)]
struct Transformation {
    x: isize,
    y: isize,
    z: isize,
}

const TRANFORMATIONS: [Transformation; 24] = [
    // x = x
    Transformation { x: 1, y: 2, z: 3 },
    Transformation { x: 1, y: -3, z: 2 },
    Transformation { x: 1, y: -2, z: -3 },
    Transformation { x: 1, y: 3, z: -2 },
    // x = -x
    Transformation { x: -1, y: -2, z: 3 },
    Transformation { x: -1, y: 3, z: 2 },
    Transformation { x: -1, y: 2, z: -3 },
    Transformation {
        x: -1,
        y: -3,
        z: -2,
    },
    // x = y
    Transformation { x: 2, y: -1, z: 3 },
    Transformation { x: 2, y: 3, z: 1 },
    Transformation { x: 2, y: 1, z: -3 },
    Transformation { x: 2, y: -3, z: -1 },
    // x = -y
    Transformation { x: -2, y: 1, z: 3 },
    Transformation { x: -2, y: 3, z: -1 },
    Transformation {
        x: -2,
        y: -1,
        z: -3,
    },
    Transformation { x: -2, y: -3, z: 1 },
    // x = z
    Transformation { x: 3, y: 2, z: -1 },
    Transformation { x: 3, y: 1, z: 2 },
    Transformation { x: 3, y: -2, z: 1 },
    Transformation { x: 3, y: -1, z: -2 },
    // x = -z
    Transformation { x: -3, y: 2, z: 1 },
    Transformation { x: -3, y: -1, z: 2 },
    Transformation {
        x: -3,
        y: -2,
        z: -1,
    },
    Transformation { x: -3, y: 1, z: -2 },
];

fn parse_input() -> Vec<Scanner> {
    utils::group_lines(&INPUT)
        .iter()
        .map(|g| {
            let mut beacons = AHashSet::new();
            for line in g[1..].iter() {
                let mut parts = line.split(',');
                beacons.insert(Beacon {
                    x: parts.parse_next().unwrap(),
                    y: parts.parse_next().unwrap(),
                    z: parts.parse_next().unwrap(),
                });
            }
            Scanner { beacons }
        })
        .collect()
}

fn get_dimension(b: &Beacon, t: isize) -> isize {
    let sign = t / t.abs();
    match t.abs() {
        1 => sign * b.x,
        2 => sign * b.y,
        3 => sign * b.z,
        _ => unreachable!(),
    }
}

fn transform_beacon(b: &Beacon, ref_b: &Beacon, t: &Transformation) -> Beacon {
    Beacon {
        x: get_dimension(b, t.x) - ref_b.x,
        y: get_dimension(b, t.y) - ref_b.y,
        z: get_dimension(b, t.z) - ref_b.z,
    }
}

fn has_overlap_with_tranform(s1: &Scanner, s2: &Scanner, t: &Transformation) -> Option<Beacon> {
    for b1 in s1.beacons.iter() {
        for start_b in s2.beacons.iter() {
            let mut matches = Vec::new();
            let offset = transform_beacon(start_b, b1, t);
            for b2 in s2.beacons.iter() {
                let new_b = transform_beacon(b2, &offset, t);
                if s1.beacons.contains(&new_b) {
                    matches.push(*b2);
                }
            }
            if matches.len() >= 12 {
                return Some(Beacon {
                    x: -offset.x,
                    y: -offset.y,
                    z: -offset.z,
                });
            }
        }
    }
    None
}

fn has_overlap(s1: &Scanner, s2: &Scanner) -> Option<(Transformation, Beacon)> {
    for t in TRANFORMATIONS.iter() {
        if let Some(b) = has_overlap_with_tranform(s1, s2, t) {
            return Some((*t, b));
        }
    }
    None
}

fn find_all_beacons(scanners: &[Scanner]) -> (AHashSet<Beacon>, Vec<Beacon>) {
    let mut all_beacons = AHashSet::from_iter(scanners[0].beacons.iter().cloned());
    let mut relative_to_first = vec![Scanner::default(); scanners.len()];
    relative_to_first[0] = scanners[0].clone();

    let mut offsets = vec![Beacon::default(); scanners.len()];

    let mut to_scan = AHashSet::from_iter([0]);
    let mut remaining = AHashSet::from_iter(1..scanners.len());
    loop {
        let mut next_to_scan = AHashSet::new();
        let mut next_remaining = AHashSet::new();
        for &i in to_scan.iter() {
            for &j in remaining.iter() {
                let s2 = &scanners[j];
                if let Some((t, offset)) = has_overlap(&relative_to_first[i], s2) {
                    let mut relative_s = Scanner {
                        beacons: AHashSet::new(),
                    };
                    for b in s2.beacons.iter() {
                        let new_b = Beacon {
                            x: offset.x + get_dimension(b, t.x),
                            y: offset.y + get_dimension(b, t.y),
                            z: offset.z + get_dimension(b, t.z),
                        };
                        relative_s.beacons.insert(new_b);
                        all_beacons.insert(new_b);
                    }
                    relative_to_first[j] = relative_s;
                    offsets[j] = offset;

                    next_remaining.remove(&j);
                    next_to_scan.insert(j);
                } else if !next_to_scan.contains(&j) {
                    next_remaining.insert(j);
                }
            }
        }
        to_scan = next_to_scan;
        remaining = next_remaining;
        if remaining.is_empty() {
            break;
        }
    }
    (all_beacons, offsets)
}

pub fn part1() -> usize {
    let scanners = parse_input();
    let (all_beacons, _) = find_all_beacons(&scanners);
    all_beacons.len()
}

pub fn part2() -> usize {
    let scanners = parse_input();
    let (_, offsets) = find_all_beacons(&scanners);
    let mut max_dist = 0;
    for i in 0..offsets.len() {
        let &Beacon {
            x: i_x,
            y: i_y,
            z: i_z,
        } = &offsets[i];
        for j in (i + 1)..offsets.len() {
            let &Beacon {
                x: j_x,
                y: j_y,
                z: j_z,
            } = &offsets[j];
            let dist = ((i_x - j_x).abs() + (i_y - j_y).abs() + (i_z - j_z).abs()) as usize;
            max_dist = max_dist.max(dist);
        }
    }
    max_dist
}
