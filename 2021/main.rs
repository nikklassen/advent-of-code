#![allow(dead_code)]
#![feature(test, array_windows, new_uninit, once_cell, destructuring_assignment)]

extern crate ahash;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate shared;
extern crate test;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

use std::time::Instant;

fn main() {
    let mut start = Instant::now();
    println!("part1: {}", day07::part1());
    println!("elapsed: {:?}", Instant::now().duration_since(start));

    start = Instant::now();
    println!("part2: {}", day07::part2());
    println!("elapsed: {:?}", Instant::now().duration_since(start));
}
