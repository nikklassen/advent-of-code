use std::collections::HashSet;
use std::iter::once;

use itertools::Itertools;
use shared::utils;

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day08");
    static ref INPUT: Vec<String> = utils::read_input_lines("day08");
}

fn parse_input() -> Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)> {
    INPUT
        .iter()
        .map(|line| {
            let mut parts = line.split(" | ");
            let inputs = parts.next().unwrap();
            let input_signals = inputs
                .split_whitespace()
                .map(|signal| signal.chars().collect())
                .collect();
            let outputs = parts.next().unwrap();
            let output_signals = outputs
                .split_whitespace()
                .map(|signal| signal.chars().collect())
                .collect();
            (input_signals, output_signals)
        })
        .collect()
}

fn simple_digit_test(len: usize) -> Option<usize> {
    match len {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}

pub fn part1() -> usize {
    parse_input()
        .iter()
        .map(|(_, outputs)| {
            outputs
                .iter()
                .filter_map(|output| simple_digit_test(output.len()))
                .count()
        })
        .sum()
}

fn get_segments(i: usize) -> HashSet<usize> {
    match i {
        0 => HashSet::from_iter(vec![0, 1, 2, 4, 5, 6].iter().cloned()),
        1 => HashSet::from_iter(vec![2, 6].iter().cloned()),
        2 => HashSet::from_iter(vec![1, 2, 3, 4, 5].iter().cloned()),
        3 => HashSet::from_iter(vec![1, 2, 3, 6, 5].iter().cloned()),
        4 => HashSet::from_iter(vec![0, 3, 2, 6].iter().cloned()),
        5 => HashSet::from_iter(vec![1, 0, 3, 6, 5].iter().cloned()),
        6 => HashSet::from_iter(vec![1, 0, 3, 4, 5, 6].iter().cloned()),
        7 => HashSet::from_iter(vec![1, 2, 6].iter().cloned()),
        8 => HashSet::from_iter(vec![0, 1, 2, 3, 4, 5, 6].iter().cloned()),
        9 => HashSet::from_iter(vec![0, 1, 3, 2, 6, 5].iter().cloned()),
        _ => unreachable!(),
    }
}

fn which_num(display: &[char; 7], value: &HashSet<char>) -> usize {
    for i in 0..=9 {
        let seg_chars: HashSet<char> = get_segments(i).iter().map(|&d| display[d]).collect();
        if seg_chars.len() == value.len() && seg_chars.difference(&value).count() == 0 {
            return i;
        }
    }
    unreachable!()
}

//  1
// 0 2
//  3
// 4 6
//  5

fn map_display(inputs: &Vec<HashSet<char>>) -> [char; 7] {
    let mut display = [' '; 7];
    let mut values = vec![HashSet::new(); 10];
    for input in inputs.iter() {
        if let Some(d) = simple_digit_test(input.len()) {
            values[d] = input.clone();
        }
    }

    let one_values = inputs.iter().find(|input| input.len() == 2).unwrap();

    let four_values = inputs.iter().find(|input| input.len() == 4).unwrap();

    let seven_values = inputs.iter().find(|input| input.len() == 3).unwrap();

    display[1] = *seven_values.difference(&one_values).next().unwrap();

    let three_values = inputs
        .iter()
        .find(|input| input.len() == 5 && input.intersection(&one_values).count() == 2)
        .unwrap();

    display[5] = *three_values
        .difference(&seven_values.union(&four_values).cloned().collect())
        .next()
        .unwrap();

    display[3] = *three_values
        .difference(
            &seven_values
                .iter()
                .cloned()
                .chain(once(display[5]))
                .collect(),
        )
        .next()
        .unwrap();

    let nine_values = inputs
        .iter()
        .find(|input| {
            input.len() == 6 && input.intersection(&three_values).count() == three_values.len()
        })
        .unwrap();

    display[0] = *nine_values.difference(&three_values).next().unwrap();

    let eight_values = inputs.iter().find(|input| input.len() == 7).unwrap();

    display[4] = *eight_values.difference(&nine_values).next().unwrap();

    let two_and_five_values = inputs
        .iter()
        .filter(|input| {
            input.len() == 5 && input.intersection(&three_values).count() != three_values.len()
        })
        .collect_vec();

    let two_values: HashSet<char>;
    let five_values: HashSet<char>;
    if two_and_five_values[0].contains(&display[4]) {
        two_values = two_and_five_values[0].clone();
        five_values = two_and_five_values[1].clone();
    } else {
        five_values = two_and_five_values[0].clone();
        two_values = two_and_five_values[1].clone();
    }

    display[2] = *two_values.intersection(&one_values).next().unwrap();
    display[6] = *five_values.intersection(&one_values).next().unwrap();

    display
}

pub fn part2() -> usize {
    let mut sum = 0;
    for (inputs, outputs) in parse_input().iter() {
        let display = map_display(inputs);

        let mut n = 0;
        for output in outputs.iter() {
            n *= 10;
            n += which_num(&display, &output);
        }

        sum += n;
    }
    sum
}
