use pathfinding::prelude::Matrix;
use shared::utils;

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day20");
    static ref INPUT: Vec<String> = utils::read_input_lines("day20");
}

fn parse_input() -> (Matrix<bool>, Vec<bool>) {
    let enhance_map = INPUT[0]
        .chars()
        .map(|c| if c == '.' { false } else { true })
        .collect();

    let mut m = Matrix::new(INPUT.len() - 2, INPUT[2].len(), false);
    for (row, line) in INPUT[2..].iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                m[(row, col)] = true;
            }
        }
    }
    (m, enhance_map)
}

fn enhance_pixel(
    m: &Matrix<bool>,
    p: (isize, isize),
    outside_is_lit: bool,
    enhance_map: &[bool],
) -> bool {
    let mut map_idx = 0;
    for row_offset in (-1)..=1 {
        for col_offset in (-1)..=1 {
            let row = p.0 + row_offset;
            let col = p.1 + col_offset;
            let sub_pixel = if row < 0 || col < 0 {
                outside_is_lit
            } else {
                *m.get((row as usize, col as usize))
                    .unwrap_or(&outside_is_lit)
            };
            map_idx <<= 1;
            map_idx |= if sub_pixel { 1 } else { 0 };
        }
    }
    enhance_map[map_idx]
}

fn enhance(m: &Matrix<bool>, outside_is_lit: bool, enhance_map: &[bool]) -> (Matrix<bool>, bool) {
    let mut new_m = Matrix::new(m.rows + 2, m.columns + 2, false);

    for row in (-1)..=(m.rows as isize) {
        for col in (-1)..=(m.columns as isize) {
            new_m[((row + 1) as usize, (col + 1) as usize)] =
                enhance_pixel(m, (row, col), outside_is_lit, enhance_map);
        }
    }

    (
        new_m,
        if outside_is_lit {
            enhance_map[511]
        } else {
            enhance_map[0]
        },
    )
}

fn print_img(m: &Matrix<bool>) {
    for row in m.iter() {
        for v in row.iter() {
            print!("{}", if *v { '#' } else { '.' });
        }
        println!("");
    }
}

fn multi_enhance(n: usize) -> usize {
    let (mut m, enhance_map) = parse_input();
    let mut outside_is_lit = false;
    for _ in 0..n {
        let r = enhance(&m, outside_is_lit, &enhance_map);
        m = r.0;
        outside_is_lit = r.1;
    }
    let mut c = 0;
    for v in m.values() {
        if *v {
            c += 1;
        }
    }
    c
}

pub fn part1() -> usize {
    multi_enhance(2)
}

pub fn part2() -> usize {
    multi_enhance(50)
}
