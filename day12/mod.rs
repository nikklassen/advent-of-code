use crate::utils;

enum Action {
    North(i32),
    East(i32),
    South(i32),
    West(i32),
    Left(f32),
    Right(f32),
    Forward(i32),
}

#[derive(Debug)]
struct Position {
    east: i32,
    north: i32,
}

struct BoatState {
    position: Position,
    direction: f32,
}

#[derive(Debug)]
struct WaypointState {
    boat: Position,
    direction: Position,
}

static PI_2: f32 = 2.0 * std::f32::consts::PI;

fn read_actions() -> Vec<Action> {
    utils::read_input_lines("day12")
        .iter()
        .map(|line| {
            let mut chars = line.chars();
            let command = chars.next().unwrap();
            let val: i32 = chars.collect::<String>().parse().unwrap();
            match command {
                'N' => Action::North(val),
                'E' => Action::East(val),
                'S' => Action::South(val),
                'W' => Action::West(val),
                'L' => Action::Left((val as f32).to_radians()),
                'R' => Action::Right((val as f32).to_radians()),
                'F' => Action::Forward(val),
                _ => panic!("bad action: {}", command),
            }
        })
        .collect()
}

fn add_angle(base: f32, other: f32) -> f32 {
    let new = base + other;
    if new < 0.0 {
        return new + PI_2;
    }
    if new > PI_2 {
        return new - PI_2;
    }
    new
}

fn next_state_boat(state: BoatState, action: &Action) -> BoatState {
    match action {
        Action::North(val) => BoatState {
            position: Position {
                north: state.position.north + val,
                east: state.position.east,
            },
            ..state
        },
        Action::East(val) => BoatState {
            position: Position {
                north: state.position.north,
                east: state.position.east + val,
            },
            ..state
        },
        Action::South(val) => BoatState {
            position: Position {
                north: state.position.north - val,
                east: state.position.east,
            },
            ..state
        },
        Action::West(val) => BoatState {
            position: Position {
                north: state.position.north,
                east: state.position.east - val,
            },
            ..state
        },
        Action::Left(val) => BoatState {
            direction: add_angle(state.direction, *val),
            ..state
        },
        Action::Right(val) => BoatState {
            direction: add_angle(state.direction, -val),
            ..state
        },
        &Action::Forward(val) => {
            let (north_v, east_v) = state.direction.sin_cos();
            let north = ((val as f32) * north_v).round() as i32;
            let east = ((val as f32) * east_v).round() as i32;
            BoatState {
                position: Position {
                    north: state.position.north + north,
                    east: state.position.east + east,
                },
                ..state
            }
        }
    }
}

pub fn part1() -> i32 {
    let final_state = read_actions().iter().fold(
        BoatState {
            position: Position { north: 0, east: 0 },
            direction: 0.0,
        },
        next_state_boat,
    );
    final_state.position.north.abs() + final_state.position.east.abs()
}

fn rotate_angle(x: f32, y: f32, angle: f32) -> (f32, f32) {
    let start_angle = y.atan2(x);
    let len = y.hypot(x);
    let (north_v, east_v) = add_angle(start_angle, angle).sin_cos();
    (len * east_v, len * north_v)
}

fn next_state_waypoint(state: WaypointState, action: &Action) -> WaypointState {
    match action {
        Action::North(val) => WaypointState {
            boat: Position {
                north: state.boat.north + val,
                east: state.boat.east,
            },
            ..state
        },
        Action::East(val) => WaypointState {
            boat: Position {
                north: state.boat.north,
                east: state.boat.east + val,
            },
            ..state
        },
        Action::South(val) => WaypointState {
            boat: Position {
                north: state.boat.north - val,
                east: state.boat.east,
            },
            ..state
        },
        Action::West(val) => WaypointState {
            boat: Position {
                north: state.boat.north,
                east: state.boat.east - val,
            },
            ..state
        },
        &Action::Left(val) => {
            let (east, north) = rotate_angle(
                state.direction.east as f32,
                state.direction.north as f32,
                val,
            );
            WaypointState {
                direction: Position {
                    north: north.round() as i32,
                    east: east.round() as i32,
                },
                ..state
            }
        }
        &Action::Right(val) => {
            let (east, north) = rotate_angle(
                state.direction.east as f32,
                state.direction.north as f32,
                -val,
            );
            WaypointState {
                direction: Position {
                    north: north.round() as i32,
                    east: east.round() as i32,
                },
                ..state
            }
        }
        &Action::Forward(val) => WaypointState {
            boat: Position {
                north: state.boat.north + state.direction.north * val,
                east: state.boat.east + state.direction.east * val,
            },
            ..state
        },
    }
}

pub fn part2() -> i32 {
    let final_state = read_actions().iter().fold(
        WaypointState {
            boat: Position { north: 0, east: 0 },
            direction: Position { north: 1, east: 10 },
        },
        next_state_waypoint,
    );
    final_state.boat.north.abs() + final_state.boat.east.abs()
}
