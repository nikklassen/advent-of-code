package main

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestIntersect(t *testing.T) {
	tests := []struct {
		name       string
		a, b, want partCondition
	}{
		{
			name: "single overlap",
			a: partCondition{
				"x": {{End: 10}},
			},
			b: partCondition{
				"x": {{Start: 5, End: 20}},
			},
			want: partCondition{
				"x": {{Start: 5, End: 10}},
				"m": {{End: maxValue}},
				"a": {{End: maxValue}},
				"s": {{End: maxValue}},
			},
		},
		{
			name: "double overlap",
			a: partCondition{
				"x": {{End: 10}},
				"m": {{End: 10}},
			},
			b: partCondition{
				"x": {{Start: 5, End: 20}},
				"m": {{Start: 5, End: 20}},
			},
			want: partCondition{
				"x": {{Start: 5, End: 10}},
				"m": {{Start: 5, End: 10}},
				"a": {{End: maxValue}},
				"s": {{End: maxValue}},
			},
		},
		{
			name: "no overlap",
			a: partCondition{
				"x": {{End: 10}},
			},
			b: partCondition{
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

func TestFlip(t *testing.T) {
	r := parseRule("x>8:dest")
	want := parseRule("x<9:dest")
	if got := flip(r); got != want {
		t.Errorf("flip(%v) got %v, want %v", r, got, want)
	}
	r = parseRule("x<8:dest")
	want = parseRule("x>7:dest")
	if got := flip(r); got != want {
		t.Errorf("flip(%v) got %v, want %v", r, got, want)
	}
}

func TestDifference(t *testing.T) {
	a := partCondition{
		"x": {{End: 10}},
	}
	b := partCondition{
		"x": {{Start: 5, End: 10}},
	}
	d := difference(a, b)
	want := []partCondition{{
		"x": {{End: 5}},
	}}
	if !cmp.Equal(want, d) {
		t.Errorf("difference(%s, %s) got %s, want %s", a, b, d, want)
	}
	a = partCondition{
		"x": {{End: 10}},
		"m": {{End: 10}},
	}
	b = partCondition{
		"x": {{Start: 5, End: 10}},
		"m": {{Start: 5, End: 10}},
	}
	d = difference(a, b)
	want = []partCondition{
		{"x": {{End: 5}}, "m": {{End: 5}}},
		{"x": {{End: 5}}, "m": {{Start: 5, End: 10}}},
		{"x": {{Start: 5, End: 10}}, "m": {{Start: 0, End: 5}}},
	}
	if !cmp.Equal(want, d) {
		t.Errorf("difference(%s, %s) got %s, want %s", a, b, d, want)
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
