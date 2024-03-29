use itertools::Itertools;
use pathfinding::directed::astar;
use shared::{
    grid::{Grid, GridIndex, CARDINAL_DIRS},
    utils,
};

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day15");
    static ref INPUT: Vec<String> = utils::read_input_lines("day15");
}

fn parse_input() -> Grid<usize> {
    let char_grid = INPUT
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec();
    Grid::from_vec(char_grid)
}

fn run(grid: &Grid<usize>, source: GridIndex, target: GridIndex) -> usize {
    astar::astar(
        &source,
        |&p| {
            CARDINAL_DIRS
                .iter()
                .filter_map(|&d| grid.add_offset(p, d).map(|n| (n, grid[n])))
                .collect::<Vec<_>>()
        },
        |p| (target.0 - p.0) + (target.1 - p.1),
        |&p| p == target,
    )
    .unwrap()
    .1
}

pub fn part1() -> usize {
    let grid = parse_input();
    run(
        &grid,
        GridIndex(0, 0),
        GridIndex(grid.width() - 1, grid.height() - 1),
    )
}

fn expand_grid(g: Grid<usize>, factor: usize) -> Grid<usize> {
    let mut bigger_grid = Grid::with_bounds(g.width() * factor, g.height() * factor);
    for i in 0..factor {
        for j in 0..factor {
            for idx in g.indexes() {
                let big_idx = GridIndex(i * g.width() + idx.0, j * g.height() + idx.1);
                bigger_grid[big_idx] = ((g[idx] + i + j - 1) % 9) + 1;
            }
        }
    }
    bigger_grid
}

pub fn part2() -> usize {
    let mut grid = parse_input();
    grid = expand_grid(grid, 5);
    run(
        &grid,
        GridIndex(0, 0),
        GridIndex(grid.width() - 1, grid.height() - 1),
    )
}
