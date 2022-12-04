package main

import "testing"

func TestPart1(t *testing.T) {
	want := 72602
	got := part1(input)
	if got != want {
		t.Errorf("part() got %d, want %d", got, want)
	}
}

func TestPart2(t *testing.T) {
	want := 207410
	got := part2(input)
	if got != want {
		t.Errorf("part() got %d, want %d", got, want)
	}
}
