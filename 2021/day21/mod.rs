use std::ops::Add;

use ahash::AHashMap;
use shared::utils::{self, NextParser};

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day21");
    static ref INPUT: Vec<String> = utils::read_input_lines("day21");
}

fn parse_input() -> (usize, usize) {
    let pos1 = INPUT[0].split(": ").skip(1).parse_next().unwrap();
    let pos2 = INPUT[1].split(": ").skip(1).parse_next().unwrap();
    (pos1, pos2)
}

fn roll<I>(die: &mut I) -> usize
where
    I: Iterator<Item = usize>,
{
    die.next().unwrap() + die.next().unwrap() + die.next().unwrap()
}

fn play<I>(mut pos1: usize, mut pos2: usize, mut die: I) -> usize
where
    I: Iterator<Item = usize>,
{
    let mut score1 = 0;
    let mut score2 = 0;
    let mut rolls = 0;
    loop {
        pos1 = ((pos1 - 1 + roll(&mut die)) % 10) + 1;
        rolls += 1;
        score1 += pos1;

        if score1 >= 1000 {
            return score2 * rolls * 3;
        }

        pos2 = ((pos2 - 1 + roll(&mut die)) % 10) + 1;
        rolls += 1;
        score2 += pos2;

        if score2 >= 1000 {
            return score1 * rolls * 3;
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Player {
    pos: usize,
    score: usize,
}

#[derive(Copy, Clone)]
struct Wins {
    player1: usize,
    player2: usize,
}

impl Add for Wins {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Wins {
            player1: self.player1 + rhs.player1,
            player2: self.player2 + rhs.player2,
        }
    }
}

fn repeat_wins(wins: Wins, times: usize) -> Wins {
    Wins {
        player1: wins.player1 * times,
        player2: wins.player2 * times,
    }
}

fn play2(
    memo: &mut AHashMap<(Player, Player, usize), Wins>,
    player1: Player,
    player2: Player,
    player: usize,
) -> Wins {
    let key = (player1, player2, player);
    if let Some(v) = memo.get(&key) {
        return *v;
    }

    if player1.score >= 21 {
        return Wins {
            player1: 1,
            player2: 0,
        };
    }
    if player2.score >= 21 {
        return Wins {
            player1: 0,
            player2: 1,
        };
    }

    let mut wins = Wins {
        player1: 0,
        player2: 0,
    };
    for (val, times) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let new_wins = if player == 2 {
            let pos = ((player2.pos - 1 + val) % 10) + 1;
            play2(
                memo,
                player1,
                Player {
                    pos,
                    score: player2.score + pos,
                },
                1,
            )
        } else {
            let pos = ((player1.pos - 1 + val) % 10) + 1;
            play2(
                memo,
                Player {
                    pos,
                    score: player1.score + pos,
                },
                player2,
                2,
            )
        };
        wins = wins + repeat_wins(new_wins, times);
    }
    memo.insert(key, wins);
    wins
}

pub fn part1() -> usize {
    let (pos1, pos2) = parse_input();
    let det_die = (1..=100).cycle();
    play(pos1, pos2, det_die)
}

pub fn part2() -> usize {
    let (pos1, pos2) = parse_input();
    let mut memo = AHashMap::new();
    let wins = play2(
        &mut memo,
        Player {
            pos: pos1,
            score: 0,
        },
        Player {
            pos: pos2,
            score: 0,
        },
        1,
    );
    wins.player1.max(wins.player2)
}
