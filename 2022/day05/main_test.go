package main

import "testing"

func TestPart1(t *testing.T) {
	want := "FJSRQCFTN"
	got := part1(input)
	if got != want {
		t.Errorf("part() got %s, want %s", got, want)
	}
}

func TestPart2(t *testing.T) {
	want := "CJVLJQPHS"
	got := part2(input)
	if got != want {
		t.Errorf("part() got %s, want %s", got, want)
	}
}
