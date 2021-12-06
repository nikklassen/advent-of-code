use shared::utils;

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day02");
    static ref INPUT: Vec<String> = utils::read_input_lines("day02");
}

#[derive(Debug)]
enum Command {
    Forward(usize),
    Up(usize),
    Down(usize),
}

fn parse_input() -> Vec<Command> {
    INPUT
        .iter()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let cmd = parts.next().unwrap();
            let meters = parts.next().unwrap().parse::<usize>().unwrap();
            match cmd {
                "forward" => Command::Forward(meters),
                "up" => Command::Up(meters),
                "down" => Command::Down(meters),
                _ => unreachable!(),
            }
        })
        .collect()
}

struct Position {
    pos: usize,
    depth: usize,
}

pub fn part1() -> usize {
    let Position { pos, depth } = parse_input().iter().fold(
        Position { pos: 0, depth: 0 },
        |Position { pos, depth }, cmd| match cmd {
            Command::Forward(m) => Position {
                pos: pos + m,
                depth,
            },
            Command::Up(m) => Position {
                pos,
                depth: depth - m,
            },
            Command::Down(m) => Position {
                pos,
                depth: depth + m,
            },
        },
    );
    pos * depth
}

#[derive(Debug)]
struct AimPosition {
    pos: usize,
    depth: usize,
    aim: usize,
}

pub fn part2() -> usize {
    let AimPosition { pos, depth, aim: _ } = parse_input().iter().fold(
        AimPosition {
            pos: 0,
            depth: 0,
            aim: 0,
        },
        |acc @ AimPosition { pos, depth, aim }, cmd| match cmd {
            Command::Forward(m) => AimPosition {
                pos: pos + m,
                depth: depth + aim * m,
                ..acc
            },
            Command::Up(m) => AimPosition {
                aim: aim - m,
                ..acc
            },
            Command::Down(m) => AimPosition {
                aim: aim + m,
                ..acc
            },
        },
    );
    pos * depth
}
