use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

use itertools::Itertools;
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

#[derive(Eq, PartialEq)]
struct ValWithPriority {
    val: GridIndex,
    priority: usize,
}

impl Ord for ValWithPriority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Reverse(self.priority).cmp(&Reverse(other.priority))
    }
}

impl PartialOrd for ValWithPriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn run(grid: &Grid<usize>, source: GridIndex, target: GridIndex) -> usize {
    let mut dist: Grid<usize> = Grid::with_bounds(grid.width(), grid.height());
    let mut prev: Grid<Option<GridIndex>> = Grid::with_bounds(grid.width(), grid.height());

    let mut q = BinaryHeap::new();

    for v in grid.indexes() {
        if v != source {
            dist[v] = usize::MAX;
        }
        q.push(ValWithPriority {
            val: v,
            priority: usize::MAX,
        })
    }

    'outer: while let Some(u) = q.pop() {
        for neighbour_dir in CARDINAL_DIRS.iter() {
            if let Some(v) = grid.add_offset(u.val, *neighbour_dir) {
                let alt = dist[u.val] + grid[v];
                if alt < dist[v] {
                    dist[v] = alt;
                    prev[v] = Some(u.val);
                    q.retain(|node| node.val != v);
                    q.push(ValWithPriority {
                        val: v,
                        priority: alt,
                    })
                }
                if v == target {
                    break 'outer;
                }
            }
        }
    }

    dist[target]
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
