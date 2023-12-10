package main

import "testing"

func TestPart1(t *testing.T) {
	want := TODO_PART1_SOL
	got := part1(input)
	if got != want {
		t.Errorf("part1 got %d, want %d", got, want)
	}
}

func TestPart2(t *testing.T) {
	want := TODO_PART2_SOL
	got := part2(input)
	if got != want {
		t.Errorf("part2 got %d, want %d", got, want)
	}
}
