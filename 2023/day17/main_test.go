package main

import "testing"

func BenchmarkPart1(b *testing.B) {
	for i := 0; i < b.N; i++ {
		want := 1008
		got := part1(input)
		if got != want {
			b.Errorf("part1 got %d, want %d", got, want)
		}
	}
}

// func TestPart2(t *testing.T) {
// 	want := 0
// 	got := part2(input)
// 	if got != want {
// 		t.Errorf("part2 got %d, want %d", got, want)
// 	}
// }
