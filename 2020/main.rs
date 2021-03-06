#![allow(dead_code)]
#![feature(test, array_windows, bindings_after_at, new_uninit)]

extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate test;

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
mod day15;
mod day16;
mod day17;
mod day18;

use std::time::Instant;

fn main() {
    let mut start = Instant::now();
    println!("part1: {}", day15::part1());
    println!("elapsed: {:?}", Instant::now().duration_since(start));

    start = Instant::now();
    println!("part2: {}", day15::part2());
    println!("elapsed: {:?}", Instant::now().duration_since(start));
}
