#![allow(dead_code)]

extern crate regex;
#[macro_use]
extern crate lazy_static;

pub mod utils;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() {
    println!("{}", day07::part2());
}
