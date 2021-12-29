use shared::{
    grid::{self, Grid},
    utils,
};

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day11");
    static ref INPUT: Vec<String> = utils::read_input_lines("day11");
}

fn parse_input() -> Grid<usize> {
    let cells = INPUT
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    Grid::from_vec(cells)
}

fn get_next_octs(octs: &Grid<usize>) -> Grid<usize> {
    let mut to_flash = Vec::new();
    let mut next_octs = Grid::with_bounds(octs.width(), octs.height());
    for (idx, &v) in octs.enumerate_cells() {
        if v == 9 {
            to_flash.push(idx);
        }
        next_octs[idx] = v + 1;
    }
    while !to_flash.is_empty() {
        let mut next_to_flash = Vec::new();
        for o in to_flash.iter() {
            for n in grid::ADJACENT_DIRS {
                if let Some(other) = next_octs.add_offset(*o, n) {
                    if next_octs[other] == 9 {
                        next_to_flash.push(other);
                    }
                    next_octs[other] += 1
                }
            }
        }
        to_flash = next_to_flash;
    }
    next_octs
}

fn run_with_steps(mut octs: Grid<usize>, steps: usize) -> usize {
    let mut flashes = 0;
    for _ in 0..steps {
        let mut next_octs = get_next_octs(&octs);
        for v in next_octs.iter_mut() {
            if *v > 9 {
                *v = 0;
                flashes += 1;
            }
        }
        octs = next_octs;
    }
    flashes
}

fn run_until_synced(mut octs: Grid<usize>) -> usize {
    let mut steps = 0;
    let grid_size = octs.width() * octs.height();
    loop {
        steps += 1;
        let mut next_octs = get_next_octs(&octs);
        let mut flashes = 0;
        for v in next_octs.iter_mut() {
            if *v > 9 {
                *v = 0;
                flashes += 1;
            }
        }
        if flashes == grid_size {
            return steps;
        }
        octs = next_octs;
    }
}

pub fn part1() -> usize {
    let octs = parse_input();
    run_with_steps(octs, 100)
}

pub fn part2() -> usize {
    let octs = parse_input();
    run_until_synced(octs)
}
