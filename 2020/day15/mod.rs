#![allow(clippy::just_underscores_and_digits)]
use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::mem::size_of;

unsafe fn nth(nums: *mut u32, mut n: u32, mut next: u32, mut t: u32) -> u32 {
    loop {
        let p = nums.add(next as usize);
        let j = *p;
        next = if j != 0 { t - j } else { 0 };
        *p = t;
        t += 1;
        if n == 0 {
            return next;
        }
        n -= 1;
    }
}

unsafe fn run<const N: u32>() -> u32 {
    let layout =
        Layout::from_size_align((N + 1) as usize * size_of::<u32>(), size_of::<u32>()).unwrap();
    let mem = alloc_zeroed(layout) as *mut u32;
    *mem.add(0) = 1;
    *mem.add(5) = 2;
    *mem.add(4) = 3;
    *mem.add(1) = 4;
    *mem.add(10) = 5;
    *mem.add(14) = 6;
    let x = nth(mem, N - 8, 7, 7);
    dealloc(mem as *mut u8, layout);
    x
}

pub fn part1() -> u32 {
    unsafe { run::<2020>() }
}

pub fn part2() -> u32 {
    unsafe { run::<30000000>() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn run_part1() {
        assert_eq!(part1(), 203);
    }

    #[test]
    fn run_part2() {
        assert_eq!(part2(), 9007186);
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
