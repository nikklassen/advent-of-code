use crate::utils::{self, *};

lazy_static! {
    static ref INPUT: Vec<String> = utils::read_input_lines("day11");
}

#[derive(PartialEq, Debug, Copy, Clone, Eq, Hash)]
enum State {
    Floor,
    Empty,
    Occupied,
}

struct SeatState {
    state: State,
    count: u8,
}

struct IntSet<const N: usize> {
    set: [bool; N],
    elems: [usize; N],
    len: usize,
}

impl<const N: usize> IntSet<N> {
    pub fn new() -> Self {
        IntSet {
            set: [false; N],
            elems: [0; N],
            len: 0,
        }
    }

    pub unsafe fn contains(&self, i: usize) -> bool {
        *self.set.get_unchecked(i)
    }

    pub unsafe fn insert(&mut self, i: usize) {
        let e = self.set.get_unchecked_mut(i);
        if *e {
            return;
        }
        *e = true;
        *self.elems.get_unchecked_mut(self.len) = i;
        self.len += 1;
    }

    pub unsafe fn clear(&mut self) {
        for &i in self.elems[..self.len].iter() {
            *self.set.get_unchecked_mut(i) = false;
        }
        self.len = 0;
    }

    pub fn iter(&self) -> std::slice::Iter<'_, usize> {
        self.elems[..self.len].iter()
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

fn read_seats() -> (Vec<SeatState>, usize) {
    let mut row_size = 0;
    (
        INPUT
            .iter()
            .map(|line| {
                row_size = line.len();
                line.chars().map(|c| match c {
                    '.' => SeatState {
                        state: State::Floor,
                        count: 0,
                    },
                    'L' => SeatState {
                        state: State::Empty,
                        count: 0,
                    },
                    _ => panic!("invalid char: {}", c),
                })
            })
            .flatten()
            .collect(),
        row_size,
    )
}

unsafe fn update<const N: usize>(
    grid: &mut [SeatState],
    marked: &mut Vec<usize>,
    seat_map: &[Vec<usize>],
    dirty: &mut IntSet<N>,
    seat_limit: u8,
) -> usize {
    loop {
        marked.clear();
        for &pos in dirty.iter() {
            let SeatState { state, count } = grid.get_unchecked(pos);
            if state == &State::Empty && count == &0
                || state == &State::Occupied && count >= &seat_limit
            {
                marked.push(pos);
            }
        }
        if marked.is_empty() {
            return grid.iter().filter(|s| s.state == State::Occupied).count();
        }
        dirty.clear();
        for &pos in marked.iter() {
            let is_empty;
            {
                let s = grid.get_unchecked_mut(pos);
                s.state = if s.state == State::Empty {
                    is_empty = false;
                    State::Occupied
                } else {
                    is_empty = true;
                    State::Empty
                };
            };
            for &pos in seat_map.get_unchecked(pos).iter() {
                let adj = grid.get_unchecked_mut(pos);
                if is_empty {
                    if adj.count == seat_limit {
                        dirty.insert(pos);
                    }
                    adj.count -= 1;
                } else {
                    if adj.count == seat_limit - 1 {
                        dirty.insert(pos);
                    }
                    adj.count += 1;
                }
            }
        }
    }
}

unsafe fn make_seat_list<const N: usize>(grid: &[SeatState], dirty: &mut IntSet<N>) {
    for (pos, seat) in grid.iter().enumerate() {
        if seat.state != State::Floor {
            dirty.insert(pos);
        }
    }
}

unsafe fn run_machine<const N: usize>(
    grid: &mut [SeatState],
    seat_map: &[Vec<usize>],
    seat_limit: u8,
) -> usize {
    let mut dirty = IntSet::<N>::new();
    make_seat_list(grid, &mut dirty);
    let mut marked = Vec::with_capacity(dirty.len());
    update(grid, &mut marked, seat_map, &mut dirty, seat_limit)
}

fn increment_pos(bounds: (usize, usize), pos: GridIndex, dir: GridDir) -> Option<GridIndex> {
    let i_bound = bounds.0;
    let j_bound = bounds.1;

    if pos.0 == 0 && dir.0 < 0
        || pos.0 == i_bound - 1 && dir.0 > 0
        || pos.1 == 0 && dir.1 < 0
        || pos.1 == j_bound - 1 && dir.1 > 0
    {
        None
    } else {
        Some((
            ((pos.0 as isize) + dir.0) as usize,
            ((pos.1 as isize) + dir.1) as usize,
        ))
    }
}

fn make_adjacent_seats_map(grid: &[SeatState], row_size: usize) -> Vec<Vec<usize>> {
    let mut map = vec![vec![]; grid.len()];
    let n = grid.len();
    let push_if = |adj: &mut Vec<usize>, pos: usize| {
        if grid[pos].state != State::Floor {
            adj.push(pos)
        }
    };
    for (pos, seat_cell) in grid.iter().enumerate() {
        let seat = seat_cell.state;
        if seat == State::Floor {
            continue;
        }

        let j = pos % row_size;

        let mut adjacent = Vec::with_capacity(8);
        if pos >= row_size {
            let mid = pos - row_size;
            if j > 0 {
                push_if(&mut adjacent, mid - 1);
            }
            push_if(&mut adjacent, mid);
            if j < row_size - 1 {
                push_if(&mut adjacent, mid + 1);
            }
        }
        if j > 0 {
            push_if(&mut adjacent, pos - 1);
        }
        if pos < n - row_size {
            let mid = pos + row_size;
            if j > 0 {
                push_if(&mut adjacent, mid - 1);
            }
            push_if(&mut adjacent, mid);
            if j < row_size - 1 {
                push_if(&mut adjacent, mid + 1);
            }
        }
        if j < row_size - 1 {
            push_if(&mut adjacent, pos + 1);
        }

        map[pos] = adjacent;
    }
    map
}

pub fn part1() -> usize {
    let (mut grid, row_size) = read_seats();
    let seat_map = make_adjacent_seats_map(&grid, row_size);
    unsafe { run_machine::<{ 92 * 99 }>(&mut grid, &seat_map, 4) }
}

fn make_closest_seats_map(grid: &[SeatState], row_size: usize) -> Vec<Vec<usize>> {
    let mut map = vec![vec![]; grid.len()];
    let bounds = (grid.len() / row_size, row_size);
    for (pos, seat_cell) in grid.iter().enumerate() {
        let seat = seat_cell.state;
        if seat == State::Floor {
            continue;
        }

        let i = pos / row_size;
        let j = pos % row_size;

        let mut adjacent = Vec::with_capacity(8);
        for &dir in utils::ADJACENT_DIRS.iter() {
            let mut current_pos = increment_pos(bounds, (i, j), dir);
            while let Some(p) = current_pos {
                let pos = p.0 * row_size + p.1;
                let s = &grid[pos];
                if s.state != State::Floor {
                    adjacent.push(pos);
                    break;
                }
                current_pos = increment_pos(bounds, p, dir);
            }
        }

        map[pos] = adjacent;
    }
    map
}

pub fn part2() -> usize {
    let (mut seats, row_size) = read_seats();
    let seat_map = make_closest_seats_map(&seats, row_size);
    unsafe { run_machine::<{ 92 * 99 }>(&mut seats, &seat_map, 5) }
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
