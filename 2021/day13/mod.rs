use pathfinding::prelude::Matrix;
use shared::utils::{self, NextParser};

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day13");
    static ref INPUT: Vec<String> = utils::read_input_lines("day13");
}

enum Fold {
    X(usize),
    Y(usize),
}

fn parse_input() -> (Matrix<bool>, Vec<Fold>) {
    let mut lines = INPUT.iter();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut points = vec![];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let mut parts = line.split(',');
        let x = parts.parse_next().unwrap();
        let y = parts.parse_next().unwrap();
        max_x = max_x.max(x);
        max_y = max_y.max(y);

        points.push((x, y));
    }
    let mut m = Matrix::new(max_x + 1, max_y + 1, false);
    for p in points.iter() {
        m[*p] = true;
    }
    let mut folds = vec![];
    for line in lines {
        let fold = line.split_whitespace().skip(2).next().unwrap();
        let mut parts = fold.split("=");
        if parts.next().unwrap() == "x" {
            folds.push(Fold::X(parts.parse_next().unwrap()));
        } else {
            folds.push(Fold::Y(parts.parse_next().unwrap()));
        }
    }
    (m, folds)
}

fn fold(m: &mut Matrix<bool>, (max_x, max_y): (usize, usize), f: &Fold) -> (usize, usize) {
    match f {
        Fold::X(f_x) => {
            for x in 1..(max_x - f_x) {
                for y in 0..max_y {
                    if m[(f_x + x, y)] {
                        m[(f_x + x, y)] = false;
                        m[(f_x - x, y)] = true;
                    }
                }
            }
            (*f_x, max_y)
        }
        Fold::Y(f_y) => {
            for x in 0..max_x {
                for y in 1..(max_y - f_y) {
                    if m[(x, f_y + y)] {
                        m[(x, f_y + y)] = false;
                        m[(x, f_y - y)] = true;
                    }
                }
            }
            (max_x, *f_y)
        }
    }
}

pub fn part1() -> usize {
    let (mut m, folds) = parse_input();
    let mut bounds = (m.rows, m.columns);
    bounds = fold(&mut m, bounds, &folds[0]);
    let mut c = 0;
    for x in 0..bounds.0 {
        for y in 0..bounds.1 {
            if m[(x, y)] {
                c += 1;
            }
        }
    }
    c
}

pub fn part2() -> usize {
    let (m, folds) = parse_input();
    let b = (m.rows, m.columns);
    let (mut final_m, (max_x, max_y)) = folds.iter().fold((m, b), |(mut m, mut b), f| {
        b = fold(&mut m, b, f);
        (m, b)
    });
    final_m = final_m.transposed();
    for x in 0..max_y {
        for y in 0..max_x {
            if final_m[(x, y)] {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!("");
    }
    0
}
