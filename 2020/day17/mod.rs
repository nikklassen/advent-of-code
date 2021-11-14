use crate::utils::{self, *};

#[derive(Copy, Clone, std::hash::Hash, Eq, PartialEq)]
struct Point3(isize, isize, isize);

impl From<(isize, isize)> for Point3 {
    fn from(val: (isize, isize)) -> Self {
        Point3(val.0, val.1, 0)
    }
}

impl Point for Point3 {
    fn adjacent(&self) -> Vec<Self> {
        let mut adjacent = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }
                    adjacent.push(Self(self.0 + x, self.1 + y, self.2 + z))
                }
            }
        }
        adjacent
    }
}

#[derive(Copy, Clone, std::hash::Hash, Eq, PartialEq)]
struct Point4(isize, isize, isize, isize);

impl Point for Point4 {
    fn adjacent(&self) -> Vec<Self> {
        let mut adjacent = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    for w in -1..=1 {
                        if x == 0 && y == 0 && z == 0 && w == 0 {
                            continue;
                        }
                        adjacent.push(Self(self.0 + x, self.1 + y, self.2 + z, self.3 + w))
                    }
                }
            }
        }
        adjacent
    }
}

impl From<(isize, isize)> for Point4 {
    fn from(val: (isize, isize)) -> Self {
        Point4(val.0, val.1, 0, 0)
    }
}

trait Point: Eq + std::hash::Hash + Copy + From<(isize, isize)> {
    fn adjacent(&self) -> Vec<Self>;
}

fn print_cubes(cubes: &HashSet<Point3>) {
    let mut bounds = (isize::MAX, isize::MIN);
    let mut z_bounds = (isize::MAX, isize::MIN);
    for cube in cubes.iter() {
        if cube.0 < bounds.0 {
            bounds = (cube.0, bounds.1);
        }
        if cube.0 > bounds.1 {
            bounds = (bounds.0, cube.0);
        }
        if cube.1 < bounds.0 {
            bounds = (cube.1, bounds.1);
        }
        if cube.1 > bounds.1 {
            bounds = (bounds.0, cube.1);
        }
        if cube.2 < z_bounds.0 {
            z_bounds = (cube.2, z_bounds.1);
        }
        if cube.2 > z_bounds.1 {
            z_bounds = (z_bounds.0, cube.2);
        }
    }
    for z in z_bounds.0..=z_bounds.1 {
        println!("z = {}", z);
        for x in bounds.0..=bounds.1 {
            for y in bounds.0..=bounds.1 {
                let c = if cubes.contains(&Point3(x, y, z)) {
                    "#"
                } else {
                    "."
                };
                print!("{}", c);
            }
            println!();
        }
    }
}

fn read_start<P: Point>() -> HashSet<P> {
    let mut cubes = HashSet::new();
    let lines = utils::read_input_lines("day17");
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                cubes.insert(P::from((i as isize, j as isize)));
            }
        }
    }
    cubes
}

fn transform<P: Point>(cubes: &HashSet<P>) -> (Vec<P>, Vec<P>) {
    let mut to_remove = Vec::new();
    let mut candidates = HashMap::<P, usize>::new();
    for cube in cubes.iter() {
        let (nearby_active, nearby_inactive) = cube
            .adjacent()
            .into_iter()
            .partition::<Vec<_>, _>(|p| cubes.contains(p));
        if !(nearby_active.len() == 2 || nearby_active.len() == 3) {
            to_remove.push(*cube);
        }
        for inactive in nearby_inactive.into_iter() {
            let e = candidates.entry(inactive).or_default();
            *e += 1;
        }
    }
    let to_add = candidates
        .into_iter()
        .filter_map(|(p, n)| if n == 3 { Some(p) } else { None })
        .collect();
    (to_add, to_remove)
}

fn run_machine<P: Point>() -> usize {
    let mut cubes = read_start::<P>();
    for _ in 0..6 {
        let (to_add, to_remove) = transform(&cubes);
        for p in to_add {
            cubes.insert(p);
        }
        for p in to_remove {
            cubes.remove(&p);
        }
    }
    cubes.len()
}

pub fn part1() -> usize {
    run_machine::<Point3>()
}

pub fn part2() -> usize {
    run_machine::<Point4>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn run_part1() {
        assert_eq!(part1(), 255);
    }

    #[test]
    fn run_part2() {
        assert_eq!(part2(), 2340);
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
