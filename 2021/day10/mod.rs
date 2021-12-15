use itertools::Itertools;
use shared::utils;

lazy_static! {
    // static ref INPUT: Vec<String> = utils::read_sample_input_lines("day10");
    static ref INPUT: Vec<String> = utils::read_input_lines("day10");
}

#[derive(Debug)]
enum LineResult {
    Corrupted(char),
    Incomplete(Vec<char>),
}

fn parse_input() -> Vec<String> {
    INPUT.iter().cloned().collect()
}

fn is_pair(left: char, right: char) -> bool {
    left == '(' && right == ')'
        || left == '[' && right == ']'
        || left == '{' && right == '}'
        || left == '<' && right == '>'
}

fn parse_line(line: &str) -> LineResult {
    let mut brackets = vec![];
    for c in line.chars() {
        match c {
            '(' | '<' | '[' | '{' => brackets.push(c),
            ')' | '>' | ']' | '}' => {
                let top = brackets.pop();
                if top.is_none() || !is_pair(top.unwrap(), c) {
                    return LineResult::Corrupted(c);
                }
            }
            _ => unreachable!(),
        }
    }
    LineResult::Incomplete(brackets)
}

pub fn part1() -> usize {
    let results = parse_input().iter().map(|s| parse_line(s)).collect_vec();
    results
        .iter()
        .map(|res| match res {
            LineResult::Corrupted(')') => 3,
            LineResult::Corrupted(']') => 57,
            LineResult::Corrupted('}') => 1197,
            LineResult::Corrupted('>') => 25137,
            _ => 0,
        })
        .sum()
}

fn score_incomplete(mut stack: Vec<char>) -> usize {
    let mut score = 0;
    while let Some(c) = stack.pop() {
        score = score * 5
            + match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => unreachable!(),
            };
    }
    score
}

pub fn part2() -> usize {
    let mut scores = parse_input()
        .iter()
        .map(|s| parse_line(s))
        .filter_map(|res| match res {
            LineResult::Incomplete(stack) => Some(stack),
            _ => None,
        })
        .map(score_incomplete)
        .collect_vec();
    scores.sort_unstable();
    scores[scores.len() / 2]
}
