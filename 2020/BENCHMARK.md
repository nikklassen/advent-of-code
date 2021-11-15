# Benchmark results

Note: Benchmarks do not include reading input lines into a `Vec<String>`

```
Environment: WSL 2
Cargo:       cargo 1.50.0-nightly
CPU:         i7-6700K
RAM:         16 GB


test day01::tests::bench_part_1 ... bench:       2,283 ns/iter (+/- 814)
test day01::tests::bench_part_2 ... bench:       2,429 ns/iter (+/- 762)

test day02::tests::bench_part_1 ... bench:      40,589 ns/iter (+/- 13,756)
test day02::tests::bench_part_2 ... bench:      31,580 ns/iter (+/- 10,886)

test day03::tests::bench_part_1 ... bench:      37,572 ns/iter (+/- 16,146)
test day03::tests::bench_part_2 ... bench:      40,028 ns/iter (+/- 13,257)

test day04::tests::bench_part_1 ... bench:      96,545 ns/iter (+/- 102,125)
test day04::tests::bench_part_2 ... bench:     269,693 ns/iter (+/- 97,202)

test day05::tests::bench_part_1 ... bench:      25,793 ns/iter (+/- 9,780)
test day05::tests::bench_part_2 ... bench:      36,679 ns/iter (+/- 12,626)

test day06::tests::bench_part_1 ... bench:      67,545 ns/iter (+/- 27,906)
test day06::tests::bench_part_2 ... bench:      65,509 ns/iter (+/- 32,477)

test day07::tests::bench_part_1 ... bench:     337,037 ns/iter (+/- 125,871)
test day07::tests::bench_part_2 ... bench:     255,250 ns/iter (+/- 92,427)

test day08::tests::bench_part_1 ... bench:      50,704 ns/iter (+/- 16,529)
test day08::tests::bench_part_2 ... bench:      54,209 ns/iter (+/- 18,745)

test day09::tests::bench_part_1 ... bench:     159,512 ns/iter (+/- 34,033)
test day09::tests::bench_part_2 ... bench:     173,101 ns/iter (+/- 52,507)

test day10::tests::bench_part_1 ... bench:       1,171 ns/iter (+/- 507)
test day10::tests::bench_part_2 ... bench:       1,663 ns/iter (+/- 804)

test day11::tests::bench_part_1 ... bench:      42,232 ns/iter (+/- 13,159)
test day11::tests::bench_part_2 ... bench:     128,146 ns/iter (+/- 22,260)

test day12::tests::bench_part_1 ... bench:      27,977 ns/iter (+/- 9,311)
test day12::tests::bench_part_2 ... bench:      34,429 ns/iter (+/- 13,882)

test day13::tests::bench_part_1 ... bench:       1,056 ns/iter (+/- 332)
test day13::tests::bench_part_2 ... bench:       5,245 ns/iter (+/- 1,205)

test day14::tests::bench_part_1 ... bench:      36,205 ns/iter (+/- 8,594)
test day14::tests::bench_part_2 ... bench:   3,537,035 ns/iter (+/- 1,091,371)

test day15::tests::bench_part_1 ... bench:       3,626 ns/iter (+/- 196)
test day15::tests::bench_part_2 ... bench: 511,602,880 ns/iter (+/- 23,134,400)
```

## Unoptimized results

```
test day16::tests::bench_part_1 ... bench:     181,017 ns/iter (+/- 271,255)
test day16::tests::bench_part_2 ... bench:     743,955 ns/iter (+/- 372,004)

test day17::tests::bench_part_1 ... bench:   2,551,462 ns/iter (+/- 549,742)
test day17::tests::bench_part_2 ... bench:  24,951,040 ns/iter (+/- 3,936,835)

test day18::tests::bench_part_1 ... bench:     378,090 ns/iter (+/- 93,048)
test day18::tests::bench_part_2 ... bench:     625,030 ns/iter (+/- 174,333)
```