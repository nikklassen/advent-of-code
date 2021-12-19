use shared::utils::{self, NextParser};

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day17");
    static ref INPUT: Vec<String> = utils::read_input_lines("day17");
}

fn parse_range(s: &str) -> (isize, isize) {
    let range = s.split("=").skip(1).next().unwrap();
    let mut bounds = range.split("..");
    (bounds.parse_next().unwrap(), bounds.parse_next().unwrap())
}

fn parse_input() -> ((isize, isize), (isize, isize)) {
    // [target area: ] x=x1..x2, y=y1..y2
    let mut parts = INPUT[0].split_whitespace().skip(2);
    let x_range_str = parts.next().unwrap();
    let x_bounds = parse_range(x_range_str.trim_end_matches(","));
    let y_bounds = parse_range(parts.next().unwrap());
    (x_bounds, y_bounds)
}

fn calc_max_height(
    x: isize,
    y: isize,
    target_x: (isize, isize),
    target_y: (isize, isize),
) -> Option<isize> {
    let mut dx = x;
    let mut dy = y;
    let mut x = 0;
    let mut y = 0;
    let mut max_y = isize::MIN;
    let mut v = None;
    loop {
        x += dx;
        y += dy;
        if y > max_y {
            max_y = y;
        }
        if x > target_x.1 || y < target_y.0 {
            return v;
        }
        if target_x.0 <= x && x <= target_x.1 && target_y.0 <= y && y <= target_y.1 {
            v = Some(max_y);
        }
        dx = 0.max(dx - 1);
        dy -= 1;
    }
}

struct Trajectory {
    x: isize,
    y: isize,
    max_height: isize,
}

fn find_all_trajectories() -> Vec<Trajectory> {
    let (x_bounds, y_bounds) = parse_input();
    let x_min = -0.5 + 0.5 * (1.0 + 8.0 * (x_bounds.0 as f32)).sqrt();
    let x_max = x_bounds.1;
    let y_min = y_bounds.0;

    let mut trajectories = vec![];
    for x in (x_min.ceil() as isize)..=x_max {
        for y in y_min..=100 {
            if let Some(max_height) = calc_max_height(x, y, x_bounds, y_bounds) {
                trajectories.push(Trajectory { x, y, max_height })
            }
        }
    }
    trajectories
}

pub fn part1() -> isize {
    let trajectories = find_all_trajectories();
    let t = trajectories
        .iter()
        .max_by(|t1, t2| t1.max_height.cmp(&t2.max_height))
        .unwrap();
    t.max_height
}

pub fn part2() -> usize {
    find_all_trajectories().len()
}
