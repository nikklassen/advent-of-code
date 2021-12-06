use ahash::{AHashMap, AHashSet};
use std::collections::VecDeque;

use shared::utils;

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day22");
    static ref INPUT: Vec<String> = utils::read_input_lines("day22");
}

fn parse_deck(lines: &[&String]) -> VecDeque<usize> {
    lines
        .iter()
        .skip(1)
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

fn parse_input() -> (VecDeque<usize>, VecDeque<usize>) {
    let players = utils::group_lines(&INPUT);
    let player1 = &players[0];
    let player2 = &players[1];
    (parse_deck(&player1), parse_deck(&player2))
}

fn clash(deck1: &mut VecDeque<usize>, deck2: &mut VecDeque<usize>) {
    let card1 = deck1.pop_front().unwrap();
    let card2 = deck2.pop_front().unwrap();
    if card1 >= card2 {
        deck1.push_back(card1);
        deck1.push_back(card2);
    } else {
        deck2.push_back(card2);
        deck2.push_back(card1);
    }
}

fn score_deck(deck: &[usize]) -> usize {
    deck.iter()
        .zip((1..=deck.len()).rev())
        .map(|(card, index)| card * index)
        .sum()
}

pub fn part1() -> usize {
    let (mut deck1, mut deck2) = parse_input();
    while !deck1.is_empty() && !deck2.is_empty() {
        clash(&mut deck1, &mut deck2);
    }
    let mut full_deck = if !deck1.is_empty() { deck1 } else { deck2 };
    score_deck(full_deck.make_contiguous())
}

fn game_string_state(imm_deck: &[usize], pos: usize, deck: &VecDeque<usize>) -> String {
    if pos < imm_deck.len() {
        imm_deck[pos..]
            .iter()
            .chain(deck.iter())
            .map(|&i| (i as u8) as char)
            .collect()
    } else {
        deck.iter().map(|&i| (i as u8) as char).collect()
    }
}

struct LazyDeque<'a> {
    prefix: &'a [usize],
    pos: usize,
    mem: VecDeque<usize>,
}

impl<'a> LazyDeque<'a> {
    fn new(prefix: &'a [usize]) -> Self {
        LazyDeque {
            prefix,
            pos: 0,
            mem: VecDeque::with_capacity(prefix.len()),
        }
    }

    fn pop_front(&mut self) -> usize {
        if self.pos < self.prefix.len() {
            let ret = self.prefix[self.pos];
            self.pos += 1;
            ret
        } else {
            self.mem.pop_front().unwrap()
        }
    }

    fn push_back(&mut self, v: usize) {
        self.mem.push_back(v)
    }

    fn is_empty(&self) -> bool {
        self.pos == self.prefix.len() && self.mem.is_empty()
    }

    fn len(&self) -> usize {
        (self.prefix.len() - self.pos) + self.mem.len()
    }

    fn make_contiguous(&mut self) -> &mut [usize] {
        if self.pos < self.prefix.len() {
            for i in (self.pos..self.prefix.len()).rev() {
                self.mem.push_front(self.prefix[i]);
            }
            self.pos = self.prefix.len()
        }
        self.mem.make_contiguous()
    }

    fn to_vec(&self) -> Vec<usize> {
        if self.pos < self.prefix.len() {
            self.prefix[self.pos..]
                .iter()
                .chain(self.mem.iter())
                .cloned()
                .collect()
        } else {
            self.mem.iter().cloned().collect()
        }
    }

    fn to_string(&self) -> String {
        if self.pos < self.prefix.len() {
            self.prefix[self.pos..]
                .iter()
                .chain(self.mem.iter())
                .map(|&i| (i as u8) as char)
                .collect()
        } else {
            self.mem.iter().map(|&i| (i as u8) as char).collect()
        }
    }
}

fn clash2(
    deck1_prefix: &[usize],
    deck2_prefix: &[usize],
    game_cache: &mut AHashMap<(String, String), usize>,
    depth: usize,
) -> (usize, Vec<usize>) {
    /*
    let start_state = (game_string_state(&deck1), game_string_state(&deck2));
    if let Some(res) = game_cache.get(&start_state) {
        return *res;
    }
    */

    let mut deck1 = LazyDeque::new(deck1_prefix);
    let mut deck2 = LazyDeque::new(deck2_prefix);

    let mut round_cache: AHashSet<(String, String)> = AHashSet::new();
    while !deck1.is_empty() && !deck2.is_empty() {
        let game_state = (deck1.to_string(), deck2.to_string());
        if round_cache.contains(&game_state) {
            // game_cache.insert(start_state, 1);
            return (1, deck1.to_vec());
        }
        round_cache.insert(game_state);

        // println!("Deck 1: {:?}", double_to_vec(deck1, deck1_pos, &new_deck1));
        // println!("Deck 2: {:?}", double_to_vec(deck2, deck2_pos, &new_deck2));

        let card1 = deck1.pop_front();
        let card2 = deck2.pop_front();
        if card1 > deck1.len() || card2 > deck2.len() {
            if card1 >= card2 {
                deck1.push_back(card1);
                deck1.push_back(card2);
            } else {
                deck2.push_back(card2);
                deck2.push_back(card1);
            }
            continue;
        }

        if clash2(
            &deck1.make_contiguous()[..card1],
            &deck2.make_contiguous()[..card2],
            game_cache,
            depth + 1,
        )
        .0 == 1
        {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }
    if !deck1.is_empty() {
        // game_cache.insert(start_state, 1);
        (
            1,
            if depth == 0 {
                deck1.to_vec()
            } else {
                Vec::new()
            },
        )
    } else {
        // game_cache.insert(start_state, 2);
        (
            2,
            if depth == 0 {
                deck2.to_vec()
            } else {
                Vec::new()
            },
        )
    }
}

pub fn part2() -> usize {
    let (mut deck1, mut deck2) = parse_input();
    let mut game_cache = AHashMap::new();
    let (_, winning_deck) = clash2(
        &deck1.make_contiguous()[..],
        &deck2.make_contiguous()[..],
        &mut game_cache,
        0,
    );
    score_deck(&winning_deck)
}
