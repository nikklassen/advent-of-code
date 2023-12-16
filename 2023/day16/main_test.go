package main

import "testing"

func TestPart1(t *testing.T) {
	want := 7046
	got := part1(input)
	if got != want {
		t.Errorf("part1 got %d, want %d", got, want)
	}
}

func TestPart2(t *testing.T) {
	want := 7313
	got := part2(input)
	if got != want {
		t.Errorf("part2 got %d, want %d", got, want)
	}
}
