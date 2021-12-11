use shared::{
    grid::{Grid, GridDir, GridIndex},
    utils,
};

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day05");
    static ref INPUT: Vec<String> = utils::read_input_lines("day05");
}

#[derive(Debug)]
struct LineSeg {
    start: GridIndex,
    end: GridIndex,
}

fn parse_pair(str: &str) -> GridIndex {
    let mut parts = str.split(',');
    let x_str = parts.next().unwrap();
    let y_str = parts.next().unwrap();
    GridIndex(x_str.parse().unwrap(), y_str.parse().unwrap())
}

fn parse_input(filter_out_diag: bool) -> Vec<LineSeg> {
    INPUT
        .iter()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let start_str = parts.next().unwrap();
            // Skip arrow
            parts.next();
            let end_str = parts.next().unwrap();
            LineSeg {
                start: parse_pair(start_str),
                end: parse_pair(end_str),
            }
        })
        .filter(|&LineSeg { start, end }| !filter_out_diag || start.0 == end.0 || start.1 == end.1)
        .collect()
}

fn normalize_vector(
    GridIndex(start_x, start_y): GridIndex,
    GridIndex(end_x, end_y): GridIndex,
) -> GridDir {
    let delta_x = (end_x as isize) - (start_x as isize);
    let delta_y = (end_y as isize) - (start_y as isize);
    let dir_x = if delta_x != 0 {
        delta_x / delta_x.abs()
    } else {
        0
    };
    let dir_y = if delta_y != 0 {
        delta_y / delta_y.abs()
    } else {
        0
    };
    GridDir(dir_x, dir_y)
}

fn run(filter_out_diag: bool) -> usize {
    let pairs = parse_input(filter_out_diag);
    let max_bound = pairs.iter().fold(0, |acc, &LineSeg { start, end }| {
        if start.0 > acc {
            start.0
        } else if start.1 > acc {
            start.1
        } else if end.0 > acc {
            end.0
        } else if end.1 > acc {
            end.1
        } else {
            acc
        }
    });
    let mut grid = Grid::from_elem(0, max_bound + 1);
    let mut c = 0;
    for &LineSeg { start, end } in pairs.iter() {
        let dir = normalize_vector(start, end);
        let mut p = start;
        loop {
            grid[p] += 1;
            if grid[p] == 2 {
                c += 1;
            }
            if p == end {
                break;
            }
            p = grid.add_offset(p, dir).unwrap();
        }
    }
    c
}

pub fn part1() -> usize {
    run(true)
}

pub fn part2() -> usize {
    run(false)
}
