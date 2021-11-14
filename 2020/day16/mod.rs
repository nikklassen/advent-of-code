use crate::utils;
use itertools::Itertools;
use regex::Regex;
use std::ops::RangeInclusive;

lazy_static! {
    static ref INPUT: Vec<String> = utils::read_input_lines("day16");
    static ref NOTE_RE: Regex = Regex::new(r"^(.*): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
}

struct Note {
    name: String,
    range1: RangeInclusive<usize>,
    range2: RangeInclusive<usize>,
}

impl Note {
    fn contains(&self, i: &usize) -> bool {
        self.range1.contains(i) || self.range2.contains(i)
    }
}

type Ticket = Vec<usize>;

fn parse_ticket(line: String) -> Ticket {
    line.split(',').map(|n| n.parse().unwrap()).collect()
}

fn read_info() -> (Vec<Note>, Ticket, Vec<Ticket>) {
    let groups = INPUT
        .iter()
        .map(|s| s.to_string())
        .group_by(|line| !line.is_empty());
    let mut groups = groups.into_iter();
    let notes = groups
        .next()
        .unwrap()
        .1
        .map(|note| {
            let captures = NOTE_RE.captures(&note).unwrap();
            Note {
                name: captures[1].to_string(),
                range1: RangeInclusive::new(
                    captures[2].parse().unwrap(),
                    captures[3].parse().unwrap(),
                ),
                range2: RangeInclusive::new(
                    captures[4].parse().unwrap(),
                    captures[5].parse().unwrap(),
                ),
            }
        })
        .collect();
    groups.next();
    let my_ticket = parse_ticket(groups.next().unwrap().1.nth(1).unwrap());
    groups.next();
    let other_tickets = groups.next().unwrap().1.skip(1).map(parse_ticket).collect();
    (notes, my_ticket, other_tickets)
}

fn split_tickets(other_tickets: Vec<Ticket>, notes: &[Note]) -> (Vec<Ticket>, Vec<Ticket>) {
    other_tickets.into_iter().partition(|ticket| {
        ticket
            .iter()
            .all(|num| notes.iter().any(|note| note.contains(num)))
    })
}

pub fn part1() -> usize {
    let (notes, _, other_tickets) = read_info();
    let (_, invalid_tickets) = split_tickets(other_tickets, &notes);
    invalid_tickets
        .iter()
        .map(|ticket| -> usize {
            ticket
                .iter()
                .filter(|num| !notes.iter().any(|note| note.contains(num)))
                .sum()
        })
        .sum()
}

pub fn part2() -> usize {
    let (notes, my_ticket, other_tickets) = read_info();
    let (valid_tickets, _) = split_tickets(other_tickets, &notes);

    let mut matching_notes: Vec<Option<usize>> = vec![None; notes.len()];

    // Eventually terminates because the problem has a unique solution.
    while matching_notes.iter().any(|pos| pos.is_none()) {
        for (i, note) in notes.iter().enumerate() {
            // Skip notes we've already matched.
            if matching_notes
                .iter()
                .any(|pos| pos.map(|p| p == i).unwrap_or(false))
            {
                continue;
            }

            let mut possible_positions = vec![];
            for (pos, matching_note) in matching_notes.iter().enumerate() {
                // Skip known positions.
                if matching_note.is_some() {
                    continue;
                }
                let matches_all = valid_tickets
                    .iter()
                    .all(|ticket| note.contains(&ticket[pos]));
                if matches_all {
                    possible_positions.push(pos);
                }
            }
            if possible_positions.len() == 1 {
                matching_notes[possible_positions[0]] = Some(i);
            }
        }
    }

    let mut tot = 1;
    for (i, matching_note) in matching_notes.iter().enumerate() {
        if notes[matching_note.unwrap()].name.starts_with("departure") {
            tot *= my_ticket[i];
        }
    }
    tot
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn run_part1() {
        assert_eq!(part1(), 25984);
    }

    #[test]
    fn run_part2() {
        assert_eq!(part2(), 1265347500049);
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
