use std::fmt::Display;

use shared::utils;

use shared::grid::*;

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day04");
    static ref INPUT: Vec<String> = utils::read_input_lines("day04");
}

#[derive(Debug, Default)]
struct BingoSquare {
    num: usize,
    marked: bool,
}

impl Display for BingoSquare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.marked {
            write!(f, "[{:2}] ", self.num)
        } else {
            write!(f, " {:2}  ", self.num)
        }
    }
}

fn parse_input() -> (Vec<usize>, Vec<Grid<BingoSquare>>) {
    let called_nums = INPUT[0]
        .to_string()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let boards = utils::group_lines(&INPUT[2..])
        .into_iter()
        .map(|board| {
            let mut grid = Grid::new(5);
            for (y, row) in board.iter().enumerate() {
                for (x, cell) in row.split_whitespace().enumerate() {
                    grid[GridIndex(x, y)] = BingoSquare {
                        num: cell.parse().unwrap(),
                        marked: false,
                    }
                }
            }
            grid
        })
        .collect();
    (called_nums, boards)
}

fn is_done(grid: &Grid<BingoSquare>) -> bool {
    let mut col_count = vec![0; grid.width()];
    for row in grid.rows() {
        let mut row_count = 0;
        for (x, cell) in row.iter().enumerate() {
            if cell.marked {
                col_count[x] += 1;
                row_count += 1;
            }
        }
        if row_count == grid.width() {
            return true;
        }
    }
    col_count.iter().any(|&c| c == grid.height())
}

fn update_board(called_num: usize, board: &mut Grid<BingoSquare>) {
    for square in board.iter_mut() {
        if square.num == called_num {
            square.marked = true;
            return;
        }
    }
}

fn score_board(called_num: usize, board: &Grid<BingoSquare>) -> usize {
    let sum = board
        .iter()
        .filter_map(|square| {
            if !square.marked {
                return Some(square.num);
            }
            None
        })
        .sum::<usize>();
    called_num * sum
}

pub fn part1() -> usize {
    let (called_nums, mut boards) = parse_input();
    for called_num in called_nums.iter() {
        for b in boards.iter_mut() {
            update_board(*called_num, b);
        }
        if let Some(b) = boards.iter().find(|b| is_done(b)) {
            return score_board(*called_num, b);
        }
    }
    unreachable!()
}

pub fn part2() -> usize {
    let (called_nums, mut boards) = parse_input();
    for called_num in called_nums.iter() {
        for b in boards.iter_mut() {
            update_board(*called_num, b);
        }
        if boards.len() == 1 {
            if is_done(&boards[0]) {
                return score_board(*called_num, &boards[0]);
            }
            continue;
        }
        boards.retain(|b| !is_done(b));
    }
    unreachable!()
}
