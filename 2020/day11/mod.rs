use crate::utils::{self, *};

lazy_static! {
    static ref INPUT: Vec<String> = utils::read_input_lines("day11");
}

#[derive(PartialEq, Debug)]
enum State {
    Floor,
    Empty,
    Occupied,
}

fn read_seats() -> Vec<Vec<State>> {
    INPUT
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

fn mark(
    grid: &[Vec<State>],
    seats: &[GridIndex],
    seat_map: &HashMap<GridIndex, Vec<GridIndex>>,
    seat_limit: usize,
) -> Vec<GridIndex> {
    let mut marked = Vec::with_capacity(seats.len());
    for pos in seats.iter() {
        let seat = utils::pos_index(grid, *pos);
        match seat {
            State::Empty => {
                if let Some(adj_seats) = seat_map.get(pos) {
                    if adj_seats
                        .iter()
                        .all(|a| utils::pos_index(grid, *a) != &State::Occupied)
                    {
                        marked.push(*pos);
                    }
                }
            }
            State::Occupied => {
                if let Some(adj_seats) = seat_map.get(pos) {
                    if adj_seats
                        .iter()
                        .filter(|a| utils::pos_index(grid, **a) == &State::Occupied)
                        .take(seat_limit)
                        .count()
                        == seat_limit
                    {
                        marked.push(*pos);
                    }
                }
            }
            State::Floor => unreachable!(),
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
    grid: &mut Vec<Vec<State>>,
    seat_map: &HashMap<GridIndex, Vec<GridIndex>>,
    seat_limit: usize,
) -> usize {
    let seats = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, seat)| {
                if seat != &State::Floor {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect::<Vec<_>>();
    loop {
        let marked = mark(grid, &seats, seat_map, seat_limit);
        if marked.is_empty() {
            return grid
                .iter()
                .map(|row| row.iter().filter(|&s| s == &State::Occupied).count())
                .sum();
        }
        update(grid, &marked);
    }
}

fn make_adjacent_seats_map(grid: &[Vec<State>]) -> HashMap<GridIndex, Vec<GridIndex>> {
    let mut map = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, seat) in row.iter().enumerate() {
            if seat == &State::Floor {
                continue;
            }

            let pos = (i, j);
            let adjacent = utils::ADJACENT_DIRS
                .iter()
                .filter_map(|dir| {
                    if let Some(seat) = utils::increment_pos(grid, pos, *dir) {
                        if utils::pos_index(grid, seat) != &State::Floor {
                            return Some(seat);
                        }
                    }
                    None
                })
                .collect();

            map.insert(pos, adjacent);
        }
    }
    map
}

pub fn part1() -> usize {
    let mut seats = read_seats();
    let seat_map = make_adjacent_seats_map(&seats);
    run_machine(&mut seats, &seat_map, 4)
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
    let mut seats = read_seats();
    let seat_map = make_closest_seats_map(&seats);
    run_machine(&mut seats, &seat_map, 5)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn run_part1() {
        assert_eq!(part1(), 2346);
    }

    #[test]
    fn run_part2() {
        assert_eq!(part2(), 2111);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part2);
    }
}
