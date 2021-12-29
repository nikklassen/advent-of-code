use ahash::{AHashMap, AHashSet};
use shared::utils;

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_file_lines(::std::path::Path::new("day12").join("sample_input_2.txt"));
    static ref INPUT: Vec<String> = utils::read_input_lines("day12");
}

#[derive(Hash, PartialEq, Clone, Copy, Eq, Debug)]
enum Cave {
    Start,
    Small(&'static str),
    Big(&'static str),
    End,
}

fn parse_cave(cave: &'static str) -> Cave {
    if cave == "start" {
        Cave::Start
    } else if cave == "end" {
        Cave::End
    } else if cave.chars().all(|c| c.is_uppercase()) {
        Cave::Big(cave)
    } else {
        Cave::Small(cave)
    }
}

fn parse_input() -> AHashMap<Cave, Vec<Cave>> {
    let mut paths = AHashMap::new();
    for line in INPUT.iter() {
        let mut parts = line.split('-');
        let v1 = parse_cave(parts.next().unwrap());
        let v2 = parse_cave(parts.next().unwrap());
        paths.entry(v1).or_insert(Vec::new()).push(v2);
        paths.entry(v2).or_insert(Vec::new()).push(v1);
    }
    paths
}

#[derive(Debug, Clone)]
struct Path {
    steps: Vec<Cave>,
    last_cave: Cave,
    caves: AHashSet<&'static str>,
    allow_dupe: bool,
    has_dupe: bool,
    track_steps: bool,
}

impl Path {
    fn new(allow_dupe: bool, track_steps: bool) -> Self {
        let mut p = Path {
            steps: Vec::new(),
            last_cave: Cave::Start,
            caves: AHashSet::new(),
            has_dupe: false,
            track_steps: false,
            allow_dupe,
        };
        if track_steps {
            p.track_steps = true;
            p.steps = vec![Cave::Start];
        }
        p
    }

    fn can_visit(&self, cave: &Cave) -> bool {
        match cave {
            &Cave::Start => false,
            Cave::Small(c) => {
                if self.allow_dupe && !self.has_dupe {
                    return true;
                }
                !self.caves.contains(c)
            }
            _ => true,
        }
    }

    fn add_cave(&self, cave: Cave) -> Self {
        let mut new_path = self.clone();
        new_path.last_cave = cave;
        if new_path.track_steps {
            new_path.steps.push(cave);
        }
        match cave {
            Cave::Big(_) => return new_path,
            Cave::Small(cave) => {
                if self.allow_dupe && new_path.caves.contains(cave) {
                    new_path.has_dupe = true;
                }
                new_path.caves.insert(cave);
            }
            _ => {}
        }
        new_path
    }
}

fn find_all_paths(m: &AHashMap<Cave, Vec<Cave>>, allow_dupe: bool) -> Vec<Path> {
    let mut fringe = vec![Path::new(allow_dupe, false)];
    let mut paths = vec![];
    while !fringe.is_empty() {
        let mut new_fringe = vec![];

        for path in fringe.iter() {
            for neighbour in m[&path.last_cave].iter() {
                if !path.can_visit(neighbour) {
                    continue;
                }
                let new_path = path.add_cave(*neighbour);
                if neighbour == &Cave::End {
                    paths.push(new_path);
                } else {
                    new_fringe.push(new_path);
                }
            }
        }
        fringe = new_fringe;
    }
    paths
}

pub fn part1() -> usize {
    let m = parse_input();
    let paths = find_all_paths(&m, false);
    paths.len()
}

pub fn part2() -> usize {
    let m = parse_input();
    let paths = find_all_paths(&m, true);
    paths.len()
}
