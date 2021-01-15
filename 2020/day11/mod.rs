use crate::utils::{self, *};

#[derive(PartialEq, Debug)]
enum State {
    Floor,
    Empty,
    Occupied,
}

fn read_seats() -> Vec<Vec<State>> {
    utils::read_input_lines("day11")
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => State::Floor,
                    '#' => State::Occupied,
                    'L' => State::Empty,
                    _ => panic!("invalid char: {}", c),
                })
                .collect()
        })
        .collect()
}

type AdjacentSeatsFn = dyn Fn(&[Vec<State>], GridIndex) -> Vec<GridIndex>;

fn mark(seats: &[Vec<State>], find_adjacent: &AdjacentSeatsFn, seat_limit: i32) -> Vec<GridIndex> {
    let mut marked = Vec::new();
    for i in 0..seats.len() {
        for (j, seat) in seats[i].iter().enumerate() {
            match seat {
                State::Floor => {}
                State::Empty => {
                    if find_adjacent(seats, (i, j))
                        .iter()
                        .all(|a| utils::pos_index(seats, *a) != &State::Occupied)
                    {
                        marked.push((i, j));
                    }
                }
                State::Occupied => {
                    if find_adjacent(seats, (i, j))
                        .iter()
                        .filter(|a| utils::pos_index(seats, **a) == &State::Occupied)
                        .count()
                        >= (seat_limit as usize)
                    {
                        marked.push((i, j));
                    }
                }
            }
        }
    }

    marked
}

fn update(seats: &mut Vec<Vec<State>>, marked: &[GridIndex]) {
    for &pos in marked {
        seats[pos.0][pos.1] = match utils::pos_index(seats, pos) {
            State::Empty => State::Occupied,
            State::Occupied => State::Empty,
            State::Floor => State::Floor,
        };
    }
}

fn run_machine(
    mut seats: Vec<Vec<State>>,
    adjacent_seats_fn: &AdjacentSeatsFn,
    seat_limit: i32,
) -> usize {
    loop {
        let marked = mark(&seats, adjacent_seats_fn, seat_limit);
        if marked.is_empty() {
            return seats
                .iter()
                .map(|row| row.iter().filter(|&s| s == &State::Occupied).count())
                .sum();
        }
        update(&mut seats, &marked);
    }
}

fn adjacent_seats(grid: &[Vec<State>], pos: GridIndex) -> Vec<GridIndex> {
    utils::ADJACENT_DIRS
        .iter()
        .filter_map(|dir| utils::increment_pos(grid, pos, *dir))
        .collect()
}

pub fn part1() -> usize {
    let seats = read_seats();
    run_machine(seats, &adjacent_seats, 4)
}

fn make_closest_seats_map(grid: &[Vec<State>]) -> HashMap<GridIndex, Vec<GridIndex>> {
    let mut map = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, seat) in row.iter().enumerate() {
            if seat == &State::Floor {
                continue;
            }

            let mut adjacent = Vec::new();
            let pos = (i, j);
            for &dir in utils::ADJACENT_DIRS.iter() {
                let mut current_pos = utils::increment_pos(grid, pos, dir);
                while let Some(p) = current_pos {
                    if utils::pos_index(grid, p) != &State::Floor {
                        adjacent.push(p);
                        break;
                    }
                    current_pos = utils::increment_pos(grid, p, dir);
                }
            }

            map.insert(pos, adjacent);
        }
    }
    map
}

pub fn part2() -> usize {
    let seats = read_seats();
    let seat_map = make_closest_seats_map(&seats);
    let far_adjacent_seats = move |_grid: &[Vec<State>], pos: GridIndex| -> Vec<GridIndex> {
        if let Some(adjacent) = seat_map.get(&pos) {
            adjacent.clone()
        } else {
            Vec::new()
        }
    };
    run_machine(seats, &far_adjacent_seats, 5)
}
