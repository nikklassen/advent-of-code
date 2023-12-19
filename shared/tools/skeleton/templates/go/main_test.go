package main

import "testing"

func BenchmarkPart1(b *testing.B) {
	for i := 0; i < b.N; i++ {
		want := 0
		got := part1(input)
		if got != want {
			b.Fatalf("part1 got %d, want %d", got, want)
		}
	}
}

// func BenchmarkPart2(b *testing.B) {
//   for i := 0; i < b.N; i++ {
//     want := 0
//     got := part2(input)
//     if got != want {
//       b.Fatalf("part2 got %d, want %d", got, want)
//     }
//   }
// }
