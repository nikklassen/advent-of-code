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
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

fn main() {
    println!("{}", day14::part2());
}
