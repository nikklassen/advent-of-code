package main

import "testing"

func TestPart1(t *testing.T) {
	want := 14163
	got := part1(input)
	if got != want {
		t.Errorf("part() got %d, want %d", got, want)
	}
}

func TestPart2(t *testing.T) {
	want := 12091
	got := part2(input)
	if got != want {
		t.Errorf("part() got %d, want %d", got, want)
	}
}
