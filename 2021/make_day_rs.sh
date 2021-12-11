#!/usr/bin/env bash
DAY=$1
if [[ -z $DAY ]]; then
  echo "You must specify a day number"
  exit 1
fi

mkdir day${DAY}
cat <<EOF > day${DAY}/mod.rs
use shared::utils;

lazy_static! {
    static ref INPUT: Vec<String> = utils::read_sample_input_lines("day${DAY}");
    // static ref INPUT: Vec<String> = utils::read_input_lines("day${DAY}");
}

fn parse_input() -> Vec<> {
    INPUT
        .iter()
        .map(|line| {
          // TODO
        })
        .collect()
}

pub fn part1() -> usize {
    0
}

pub fn part2() -> usize {
    0
}
EOF

touch day${DAY}/input.txt day${DAY}/sample_input.txt

prev_day_line="$(ag 'mod day' main.rs | awk 'BEGIN { FS = ":" } END { print $1 }')"

sed -i -e "${prev_day_line}a mod day${DAY};" -e "s/day[0-9]\\+::part/day${DAY}::part/" main.rs

