package main

import (
	"testing"

	"github.com/google/go-cmp/cmp"
	"github.com/nikklassen/advent-of-code/shared/utils"
)

func TestIntersect(t *testing.T) {
	tests := []struct {
		name       string
		a, b, want map[string]utils.RangeSet
	}{
		{
			name: "single overlap",
			a: map[string]utils.RangeSet{
				"x": {{End: 10}},
			},
			b: map[string]utils.RangeSet{
				"x": {{Start: 5, End: 20}},
			},
			want: map[string]utils.RangeSet{
				"x": {{Start: 5, End: 10}},
				"m": {{End: maxValue}},
				"a": {{End: maxValue}},
				"s": {{End: maxValue}},
			},
		},
		{
			name: "double overlap",
			a: map[string]utils.RangeSet{
				"x": {{End: 10}},
				"m": {{End: 10}},
			},
			b: map[string]utils.RangeSet{
				"x": {{Start: 5, End: 20}},
				"m": {{Start: 5, End: 20}},
			},
			want: map[string]utils.RangeSet{
				"x": {{Start: 5, End: 10}},
				"m": {{Start: 5, End: 10}},
				"a": {{End: maxValue}},
				"s": {{End: maxValue}},
			},
		},
		{
			name: "no overlap",
			a: map[string]utils.RangeSet{
				"x": {{End: 10}},
			},
			b: map[string]utils.RangeSet{
				"x": {{Start: 10, End: 20}},
			},
			want: nil,
		},
	}
	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			got := intersect(test.a, test.b)
			if !cmp.Equal(got, test.want) {
				t.Errorf("intersect(%v, %v) got %v, want %v", test.a, test.b, got, test.want)
			}
		})
	}
}

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
