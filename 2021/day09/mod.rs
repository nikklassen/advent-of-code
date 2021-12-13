use itertools::Itertools;
use shared::{
    grid::{Grid, GridIndex, CARDINAL_DIRS},
    utils,
};

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day09");
    static ref INPUT: Vec<String> = utils::read_input_lines("day09");
}

fn parse_input() -> Grid<usize> {
    Grid::from_vec(
        INPUT
            .iter()
            .map(|line| {
                let parts = line.split("").collect_vec();
                parts[1..parts.len() - 1]
                    .iter()
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect(),
    )
}

pub fn part1() -> usize {
    let grid = parse_input();
    let mut c = 0;
    for (idx, center) in grid.enumerate_cells() {
        let mut is_low = true;
        for &dir in CARDINAL_DIRS.iter() {
            let Some(p) = grid.add_offset(idx, dir) else { continue };
            if grid[p] <= *center {
                is_low = false;
                break;
            }
        }
        if is_low {
            c += center + 1;
        }
    }
    c
}

fn fill_basin(start: GridIndex, grid: &Grid<usize>, marked: &mut Grid<bool>) -> usize {
    let mut size = 0;
    let mut fringe = vec![start];
    marked[start] = true;
    while let Some(p) = fringe.pop() {
        size += 1;
        for &dir in CARDINAL_DIRS.iter() {
            let Some(nb) = grid.add_offset(p, dir) else { continue };
            if marked[nb] || grid[nb] == 9 {
                continue;
            }
            marked[nb] = true;
            fringe.push(nb);
        }
    }
    size
}

pub fn part2() -> usize {
    let grid = parse_input();
    let mut marked: Grid<bool> = Grid::with_bounds(grid.width(), grid.height());

    let mut sizes = vec![];
    for (idx, height) in grid.enumerate_cells() {
        if marked[idx] || *height == 9 {
            continue;
        }
        sizes.push(fill_basin(idx, &grid, &mut marked));
    }
    sizes.sort_unstable();
    sizes.reverse();
    sizes.iter().take(3).product()
}
