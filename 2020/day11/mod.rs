use shared::utils;

lazy_static! {
    static ref INPUT: Vec<String> = utils::read_input_lines("day11");
}

const WIDTH: usize = 92 + 2;
const HEIGHT: usize = 99 + 2;

const GRID_SIZE: usize = WIDTH * HEIGHT;

#[derive(PartialEq, Debug, Copy, Clone, Eq, Hash)]
enum State {
    Floor,
    Empty,
    Occupied,
}

type SeatState = u8;

unsafe fn print_grid(mut g: *mut SeatState) {
    for _ in 0..HEIGHT {
        for _ in 0..WIDTH {
            if *g & 0b10 == 0 {
                if *g & 1 == 0 {
                    print!(".");
                } else {
                    // Special state for debugging.
                    print!("*");
                }
            } else if *g & 1 == 0 {
                print!("L");
            } else {
                print!("#");
            }
            g = g.add(1);
        }
        println!();
    }
}

unsafe fn grid_pos<T>(g: *mut T, other: *mut T) -> String {
    let d = other.offset_from(g) as usize;
    format!("({}, {})", d / WIDTH, d % WIDTH)
}

unsafe fn assign_seats(seats: *mut SeatState) {
    let mut s = seats.add(WIDTH + 1);
    for i in 0..(HEIGHT - 2) {
        let mut c = INPUT[i].as_ptr();
        for _ in 0..(WIDTH - 2) {
            if *c == b'L' {
                *s += 2;
            }
            s = s.add(1);
            c = c.add(1);
        }
        s = s.add(2)
    }
}

unsafe fn mark(
    mut seat: *mut SeatState,
    end: *mut SeatState,
    mut marked: *mut usize,
    mut seat_num: usize,
    seat_limit: u8,
) -> *mut usize {
    loop {
        let s = *seat;
        if s & 0b10 != 0 {
            let is_empty = s & 1 == 0;
            let count = s >> 2;
            if is_empty && count == 0 || !is_empty && count >= seat_limit {
                *marked = seat_num;
                marked = marked.add(1);
            }
        }
        seat_num += 1;
        seat = seat.add(1);
        if seat == end {
            return marked;
        }
    }
}

unsafe fn count_occupied(mut seat: *const SeatState) -> usize {
    let mut c = 0;
    for _ in 0..GRID_SIZE {
        if *seat & 0b11 == 0b11 {
            c += 1
        }
        seat = seat.add(1);
    }
    c
}

unsafe fn run_machine(g: *mut SeatState, seat_limit: u8) -> usize {
    let m = [0; GRID_SIZE].as_mut_ptr();
    let end = g.add(GRID_SIZE - (WIDTH + 1));
    loop {
        let mut m_curr = mark(g.add(WIDTH + 1), end, m, WIDTH + 1, seat_limit);
        if m_curr == m {
            return count_occupied(g);
        }
        loop {
            m_curr = m_curr.sub(1);
            let pos = *m_curr;

            let p = g.add(pos);
            let mut s = *p;
            s ^= 1;
            *p = s;
            if s & 1 == 0 {
                let mut mid = p.sub(WIDTH);
                // Up left
                (*mid.sub(1)) -= 4;
                // Up
                (*mid) -= 4;
                // Up right
                (*mid.add(1)) -= 4;
                // Left
                (*p.sub(1)) -= 4;
                // Right
                (*p.add(1)) -= 4;
                mid = p.add(WIDTH);
                // Down left
                (*mid.sub(1)) -= 4;
                // Down
                (*mid) -= 4;
                // Down right
                (*mid.add(1)) -= 4;
            } else {
                let mut mid = p.sub(WIDTH);
                // Up left
                (*mid.sub(1)) += 4;
                // Up
                (*mid) += 4;
                // Up right
                (*mid.add(1)) += 4;
                // Left
                (*p.sub(1)) += 4;
                // Right
                (*p.add(1)) += 4;
                mid = p.add(WIDTH);
                // Down left
                (*mid.sub(1)) += 4;
                // Down
                (*mid) += 4;
                // Down right
                (*mid.add(1)) += 4;
            }
            if m_curr == m {
                break;
            }
        }
    }
}

pub fn part1() -> usize {
    unsafe {
        let grid = [0; GRID_SIZE].as_mut_ptr();
        assign_seats(grid);
        run_machine(grid, 4)
    }
}

unsafe fn find_seat(mut p: *mut SeatState, offset: usize, end: *mut SeatState) -> *mut SeatState {
    loop {
        p = p.add(offset);
        if p > end {
            return p.sub(offset);
        }
        if *p & 0b10 != 0 {
            return p;
        }
    }
}

unsafe fn find_seat_rev(
    mut p: *mut SeatState,
    offset: usize,
    begin: *mut SeatState,
) -> *mut SeatState {
    loop {
        p = p.sub(offset);
        if p < begin {
            return p.add(offset);
        }
        if *p & 0b10 != 0 {
            return p;
        }
    }
}

unsafe fn make_closest_seats_map(g: *mut SeatState, seat_map: *mut *mut SeatState) {
    let mut pos = 0;
    let grid_end = g.add(GRID_SIZE - 1);
    let h = HEIGHT as isize;
    let w = WIDTH as isize;
    for i in 0..h {
        let row_start = g.offset(i * w);
        let row_end = g.offset((i + 1) * w - 1);
        for j in 0..w {
            let g_curr = g.add(pos);
            let s = *g_curr;
            if s & 0b10 != 0 {
                let mut s_curr = seat_map.add(pos * 8);
                for i in 0..8 {
                    *s_curr.add(i) = g;
                }
                s_curr = seat_map.add(pos * 8);

                // Top left
                *s_curr = find_seat_rev(
                    g_curr,
                    WIDTH + 1,
                    g.offset((i - j).max(0) * w + (j - i).max(0)),
                );
                s_curr = s_curr.add(1);
                // Top
                *s_curr = find_seat_rev(g_curr, WIDTH, g);
                s_curr = s_curr.add(1);
                // Top right
                *s_curr = find_seat_rev(
                    g_curr,
                    WIDTH - 1,
                    g.offset((i - (w - j - 1)).max(0) * w + (j + i).min(w - 1)),
                );
                s_curr = s_curr.add(1);

                // Left
                *s_curr = find_seat_rev(g_curr, 1, row_start);
                s_curr = s_curr.add(1);
                // Right
                *s_curr = find_seat(g_curr, 1, row_end);
                s_curr = s_curr.add(1);

                // Bottom left
                *s_curr = find_seat(
                    g_curr,
                    WIDTH - 1,
                    g.offset((i + j).min(h - 1) * w + (j - (h - i - 1)).max(0)),
                );
                s_curr = s_curr.add(1);
                // Bottom
                *s_curr = find_seat(g_curr, WIDTH, grid_end);
                s_curr = s_curr.add(1);
                // Bottom right
                *s_curr = find_seat(
                    g_curr,
                    WIDTH + 1,
                    g.offset((i + (w - j - 1)).min(h - 1) * w + (j + (h - i - 1)).min(h - 1)),
                );
            }
            pos += 1;
        }
    }
}

unsafe fn run_machine_with_map<const N: usize>(
    g: *mut SeatState,
    seat_map: *mut *mut SeatState,
    seat_limit: u8,
) -> usize {
    let m = [0; N].as_mut_ptr();
    let end = g.add(GRID_SIZE - (WIDTH + 1));
    loop {
        let mut m_curr = mark(g.add(WIDTH + 1), end, m, WIDTH + 1, seat_limit);
        if m_curr == m {
            return count_occupied(g);
        }
        loop {
            m_curr = m_curr.sub(1);
            let pos = *m_curr;

            let p = g.add(pos);
            let mut s = *p;
            s ^= 1;
            *p = s;
            let mut adj = seat_map.add(pos * 8);
            if s & 1 == 0 {
                for _ in 0..8 {
                    **adj -= 4;
                    adj = adj.add(1);
                }
            } else {
                for _ in 0..8 {
                    **adj += 4;
                    adj = adj.add(1);
                }
            }
            if m_curr == m {
                break;
            }
        }
    }
}

pub fn part2() -> usize {
    unsafe {
        let grid = [0; GRID_SIZE].as_mut_ptr();
        let seat_map: *mut *mut SeatState = [std::ptr::null_mut(); GRID_SIZE * 8].as_mut_ptr();
        assign_seats(grid);
        make_closest_seats_map(grid, seat_map);
        run_machine_with_map::<GRID_SIZE>(grid, seat_map, 5)
    }
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
