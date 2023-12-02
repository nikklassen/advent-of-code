use ahash::AHashMap;
use itertools::Itertools;
use shared::utils::{self, NextParser};

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day22");
    static ref INPUT: Vec<String> = utils::read_input_lines("day22");
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Cube {
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

#[derive(Copy, Clone)]
struct CubeRange {
    on: bool,
    cube: Cube,
}

fn parse_range(s: &str) -> Option<(isize, isize)> {
    let mut parts = s[2..].split("..");
    Some((parts.parse_next()?, parts.parse_next()?))
}

fn parse_input() -> Option<Vec<CubeRange>> {
    INPUT
        .iter()
        .map(|line| {
            // format: "on x=10..12,y=10..12,z=10..12"
            let mut parts = line.split_whitespace();
            let on = parts.next()? == "on";
            let mut ranges = parts.next()?.split(",");
            Some(CubeRange {
                on,
                cube: Cube {
                    x: parse_range(ranges.next()?)?,
                    y: parse_range(ranges.next()?)?,
                    z: parse_range(ranges.next()?)?,
                },
            })
        })
        .collect()
}

fn apply_op(m: &mut AHashMap<(isize, isize, isize), bool>, op: &CubeRange) {
    let bounds = (-50isize)..=50;
    for x in (op.cube.x.0)..=(op.cube.x.1) {
        if !bounds.contains(&x) {
            continue;
        }
        for y in (op.cube.y.0)..=(op.cube.y.1) {
            if !bounds.contains(&y) {
                continue;
            }
            for z in (op.cube.z.0)..=(op.cube.z.1) {
                if !bounds.contains(&z) {
                    continue;
                }

                *m.entry((x, y, z)).or_default() = op.on;
            }
        }
    }
}

pub fn part1() -> usize {
    let ops = parse_input().unwrap();
    let m = ops.iter().fold(AHashMap::new(), |mut m, op| {
        apply_op(&mut m, op);
        m
    });
    let mut c = 0;
    for (_, v) in m.iter() {
        if *v {
            c += 1;
        }
    }
    c
}

fn validate_cube(c: Cube) -> Option<Cube> {
    if c.x.1 < c.x.0 || c.y.1 < c.y.0 || c.z.1 < c.z.0 {
        None
    } else {
        Some(c)
    }
}

fn cut_left(source: &Cube, cut: &Cube) -> Option<Cube> {
    validate_cube(Cube {
        x: (source.x.0, (cut.x.0 - 1).min(source.x.1)),
        y: source.y,
        z: source.z,
    })
}
fn cut_front(source: &Cube, cut: &Cube) -> Option<Cube> {
    validate_cube(Cube {
        x: (cut.x.0.max(source.x.0), cut.x.1.min(source.x.1)),
        y: source.y,
        z: (source.z.0, (cut.z.0 - 1).min(source.z.1)),
    })
}
fn cut_top(source: &Cube, cut: &Cube) -> Option<Cube> {
    validate_cube(Cube {
        x: (cut.x.0.max(source.x.0), cut.x.1.min(source.x.1)),
        y: ((cut.y.1 + 1).max(source.y.0), source.y.1),
        z: (cut.z.0.max(source.z.0), cut.z.1.min(source.z.1)),
    })
}
fn cut_bottom(source: &Cube, cut: &Cube) -> Option<Cube> {
    validate_cube(Cube {
        x: (cut.x.0.max(source.x.0), cut.x.1.min(source.x.1)),
        y: (source.y.0, (cut.y.0 - 1).min(source.y.1)),
        z: (cut.z.0.max(source.z.0), cut.z.1.min(source.z.1)),
    })
}
fn cut_back(source: &Cube, cut: &Cube) -> Option<Cube> {
    validate_cube(Cube {
        x: (cut.x.0.max(source.x.0), cut.x.1.min(source.x.1)),
        y: source.y,
        z: ((cut.z.1 + 1).max(source.z.0), source.z.1),
    })
}
fn cut_right(source: &Cube, cut: &Cube) -> Option<Cube> {
    validate_cube(Cube {
        x: ((cut.x.1 + 1).max(source.x.0), source.x.1),
        y: source.y,
        z: source.z,
    })
}

fn cube_vol(c: &Cube) -> usize {
    ((c.x.1 - c.x.0 + 1) as usize) * ((c.y.1 - c.y.0 + 1) as usize) * ((c.z.1 - c.z.0 + 1) as usize)
}

fn has_intersection(a: &Cube, b: &Cube) -> bool {
    a.x.0.max(b.x.0) <= a.x.1.min(b.x.1)
        && a.y.0.max(b.y.0) <= a.y.1.min(b.y.1)
        && a.z.0.max(b.z.0) <= a.z.1.min(b.z.1)
}

fn count_lit(bounds: &Cube, mut ranges: &[CubeRange]) -> u64 {
    if ranges.is_empty() {
        return cube_vol(bounds) as u64;
    }

    let other = ranges[0];
    ranges = &ranges[1..];
    if !has_intersection(bounds, &other.cube) {
        return count_lit(bounds, ranges);
    }

    let mut lit = 0;
    if let Some(left) = cut_left(bounds, &other.cube) {
        lit += count_lit(&left, ranges);
    }
    if let Some(front) = cut_front(bounds, &other.cube) {
        lit += count_lit(&front, ranges);
    }
    if let Some(top) = cut_top(bounds, &other.cube) {
        lit += count_lit(&top, ranges);
    }
    if let Some(bottom) = cut_bottom(bounds, &other.cube) {
        lit += count_lit(&bottom, ranges);
    }
    if let Some(back) = cut_back(bounds, &other.cube) {
        lit += count_lit(&back, ranges);
    }
    if let Some(right) = cut_right(bounds, &other.cube) {
        lit += count_lit(&right, ranges);
    }
    lit
}

pub fn part2() -> u64 {
    let ops = parse_input().unwrap();
    let mut lit = 0u64;
    for i in (0..ops.len()).rev() {
        if ops[i].on {
            lit += count_lit(&ops[i].cube, &ops[(i + 1)..])
        }
    }
    lit
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    fn cut_cube(c: &Cube, cut: &Cube) -> Vec<Cube> {
        [
            cut_left(c, cut),
            cut_front(c, cut),
            cut_top(c, cut),
            cut_bottom(c, cut),
            cut_back(c, cut),
            cut_right(c, cut),
        ]
        .into_iter()
        .flat_map(|c| c)
        .collect_vec()
    }

    #[test]
    fn test_cube_vol() {
        let c = Cube {
            x: (0, 3),
            y: (0, 3),
            z: (0, 3),
        };
        assert_eq!(cube_vol(&c), 64);
    }

    #[test]
    fn cut_superset() {
        let c = Cube {
            x: (1, 2),
            y: (1, 2),
            z: (1, 2),
        };
        let cut = Cube {
            x: (0, 3),
            y: (0, 3),
            z: (0, 3),
        };
        let leftover = cut_cube(&c, &cut);
        assert_eq!(leftover, vec![]);
    }

    #[test]
    fn cut_inner() {
        let c = Cube {
            x: (0, 3),
            y: (0, 3),
            z: (0, 3),
        };
        let cut = Cube {
            x: (1, 2),
            y: (1, 2),
            z: (1, 2),
        };
        let leftover = cut_cube(&c, &cut);
        assert_eq!(
            leftover,
            vec![
                // left
                Cube {
                    x: (0, 0),
                    y: (0, 3),
                    z: (0, 3),
                },
                // front
                Cube {
                    x: (1, 2),
                    y: (0, 3),
                    z: (0, 0),
                },
                // top
                Cube {
                    x: (1, 2),
                    y: (3, 3),
                    z: (1, 2),
                },
                // bottom
                Cube {
                    x: (1, 2),
                    y: (0, 0),
                    z: (1, 2),
                },
                // back
                Cube {
                    x: (1, 2),
                    y: (0, 3),
                    z: (3, 3),
                },
                // right
                Cube {
                    x: (3, 3),
                    y: (0, 3),
                    z: (0, 3),
                },
            ]
        );
    }

    #[test]
    fn cut_half() {
        let c = Cube {
            x: (0, 3),
            y: (0, 3),
            z: (0, 3),
        };
        let cut = Cube {
            x: (2, 3),
            y: (0, 3),
            z: (0, 3),
        };
        let leftover = cut_cube(&c, &cut);
        assert_eq!(
            leftover,
            vec![Cube {
                x: (0, 1),
                y: (0, 3),
                z: (0, 3),
            },]
        );
    }

    #[test]
    fn cut_outside() {
        let c = Cube {
            x: (0, 3),
            y: (0, 3),
            z: (0, 3),
        };
        let cut = Cube {
            x: (4, 5),
            y: (4, 5),
            z: (4, 5),
        };
        let leftover = cut_cube(&c, &cut);
        assert_eq!(leftover, vec![c]);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1257350313518866);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part2());
    }
}
