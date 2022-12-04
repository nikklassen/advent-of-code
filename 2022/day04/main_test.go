package main

import "testing"

func TestPart1(t *testing.T) {
	want := 498
	got := part1(input)
	if got != want {
		t.Errorf("part() got %d, want %d", got, want)
	}
}

func TestPart2(t *testing.T) {
	want := 859
	got := part2(input)
	if got != want {
		t.Errorf("part() got %d, want %d", got, want)
	}
}
