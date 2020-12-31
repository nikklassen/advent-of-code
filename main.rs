#![allow(dead_code)]

extern crate regex;

pub mod utils;

mod day01;
mod day02;
mod day03;

fn main() {
    println!("{}", day03::part2());
}
