package main

import (
	"strings"
	"testing"
)

func TestPart1(t *testing.T) {
	want := 14920
	got := part1(input)
	if got != want {
		t.Errorf("part1() got %d, want %d", got, want)
	}
}

func TestPart2(t *testing.T) {
	want := strings.TrimSpace(`
###..#..#..##...##...##..###..#..#.####.
#..#.#..#.#..#.#..#.#..#.#..#.#..#....#.
###..#..#.#....#..#.#....###..#..#...#..
#..#.#..#.#....####.#....#..#.#..#..#...
#..#.#..#.#..#.#..#.#..#.#..#.#..#.#....
###...##...##..#..#..##..###...##..####.`)
	got := part2(input)
	if got != want {
		t.Errorf("part2() got\n%s, want\n%s", got, want)
	}
}
