#![allow(dead_code)]
#![feature(
    test,
    array_windows,
    new_uninit,
    once_cell,
    let_else,
    binary_heap_retain,
    box_patterns
)]

extern crate ahash;
#[macro_use]
extern crate lazy_static;
extern crate pathfinding;
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
mod day19;
mod day24;

use std::time::Instant;

fn main() {
    let mut start = Instant::now();
    println!("part1: {}", day13::part1());
    println!("elapsed: {:?}", Instant::now().duration_since(start));

    start = Instant::now();
    println!("part2: {}", day13::part2());
    println!("elapsed: {:?}", Instant::now().duration_since(start));
}
