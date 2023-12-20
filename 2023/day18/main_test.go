package main

import (
	"testing"

	"github.com/google/go-cmp/cmp"
	"github.com/nikklassen/advent-of-code/shared/grid"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

func TestContains(t *testing.T) {
	g := newPathMap()
	g.AddSegment(grid.I(0, 5), grid.I(0, 10))
	if g.Contains(grid.I(0, 0)) {
		t.Errorf("got contains, want not")
	}
	if !g.Contains(grid.I(0, 8)) {
		t.Errorf("got not contains, want contains")
	}
	if g.Contains(grid.I(0, 12)) {
		t.Errorf("got contains, want not")
	}

	g.AddSegment(grid.I(5, 0), grid.I(10, 0))
	if g.Contains(grid.I(0, 0)) {
		t.Errorf("got contains, want not")
	}
	if !g.Contains(grid.I(8, 0)) {
		t.Errorf("got not contains, want contains")
	}
	if g.Contains(grid.I(12, 0)) {
		t.Errorf("got contains, want not")
	}
}

func TestOR(t *testing.T) {
	tests := []struct {
		a, b hLine
		want []hLine
	}{
		{
			a: hLine{4, -4, -2}, b: hLine{0, 0, 2},
			want: []hLine{{0, -4, -2}, {0, 0, 2}},
		},
		{
			a: hLine{0, 0, 2}, b: hLine{4, 2, 5},
			want: []hLine{{0, 0, 5}},
		},
	}
	for _, test := range tests {
		got := or(0, test.a, test.b)
		if !cmp.Equal(test.want, got) {
			t.Fatalf("or(%v, %v) got %v, want %v", test.a, test.b, got, test.want)
		}
		got = or(0, test.b, test.a)
		if !cmp.Equal(test.want, got) {
			t.Fatalf("or(%v, %v) got %v, want %v", test.a, test.b, got, test.want)
		}
	}
}

func TestORAll(t *testing.T) {
	tests := []struct {
		a, b []hLine
		want []hLine
	}{
		{
			a: []hLine{{4, -4, -2}, {4, 2, 5}}, b: []hLine{{0, 0, 2}},
			want: []hLine{{0, -4, -2}, {0, 0, 5}},
		},
		{
			a: []hLine{{4, -4, -2}, {4, 0, 5}}, b: []hLine{{0, -2, 0}},
			want: []hLine{{0, -4, 5}},
		},
	}
	for _, test := range tests {
		got := orAll(0, test.a, test.b)
		if !cmp.Equal(test.want, got) {
			t.Fatalf("orAll(%v, %v) got %v, want %v", test.a, test.b, got, test.want)
		}
		got = orAll(0, test.b, test.a)
		if !cmp.Equal(test.want, got) {
			t.Fatalf("orAll(%v, %v) got %v, want %v", test.a, test.b, got, test.want)
		}
	}
}

func TestXOR(t *testing.T) {
	tests := []struct {
		a, b hLine
		want []hLine
	}{
		{
			a: hLine{0, 0, 2}, b: hLine{0, 0, 6},
			want: []hLine{{0, 2, 6}},
		},
		{
			a: hLine{0, 2, 6}, b: hLine{0, 4, 6},
			want: []hLine{{0, 2, 4}},
		},
		{
			a: hLine{0, 0, 2}, b: hLine{0, 2, 6},
			want: []hLine{{0, 0, 6}},
		},
	}
	for _, test := range tests {
		got := xor(0, test.a, test.b)
		if !cmp.Equal(test.want, got) {
			t.Fatalf("xor(%v, %v) got %v, want %v", test.a, test.b, got, test.want)
		}
		got = xor(0, test.b, test.a)
		if !cmp.Equal(test.want, got) {
			t.Fatalf("xor(%v, %v) got %v, want %v", test.a, test.b, got, test.want)
		}
	}
}

func TestXORAll(t *testing.T) {
	got := xorAll(0, []hLine{{0, 2, 6}}, []hLine{{0, 0, 2}, {0, 4, 6}})
	want := []hLine{{0, 0, 4}}
	if !cmp.Equal(want, got) {
		t.Fatalf("got %v, want %v", got, want)
	}
}

func BenchmarkPart1(b *testing.B) {
	for i := 0; i < b.N; i++ {
		want := 33491
		got := part1(input)
		if got != want {
			b.Fatalf("part1 got %d, want %d", got, want)
		}
	}
}

func TestPart1(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  int
	}{
		{
			name: "8",
			// ###...
			// #.#...
			// #####.
			// ..#.#.
			// ..###.
			input: `R 2
D 4
R 2
U 2
L 4
U 2`,
			want: 17,
		},
		{
			name: "H",
			// ###.###
			// #.#.#.#
			// #.###.#
			// #.....#
			// #.###.#
			// #.#.#.#
			// ###.###
			input: `R 2
D 2
R 2
U 2
R 2
D 6
L 2
U 2
L 2
D 2
L 2
U 6`,
			want: 45,
		},
		{
			name: "E",
			// #######
			// #.....#
			// #.#####
			// #.#....
			// #.#####
			// #.....#
			// #.#####
			// #.#....
			// #.#####
			// #.....#
			// #######
			input: `R 6
D 2
L 4
D 2
R 4
D 2
L 4
D 2
R 4
D 2
L 6
U 10`,
			want: 69,
		},
		{
			name: "O",
			// ..###..
			// .##.##.
			// ##...##
			// #.....#
			// ##...##
			// .##.##.
			// ..###..
			input: `R 2
D 1
R 1
D 1
R 1
D 2
L 1
D 1
L 1
D 1
L 2
U 1
L 1
U 1
L 1
U 2
R 1
U 1
R 1
U 1`,
			want: 37,
		},
		{
			name: "J",
			// ..#######.
			// ..#.....#.
			// ..#####.#.
			// ......#.#.
			// .###..#.#.
			// .#.#..#.#.
			// .#.####.#.
			// .#......#.
			// .########.
			input: `R 6
D 8
L 7
U 4
R 2
D 2
R 3
U 4
L 4
U 2`,
			want: 60,
		},
		{
			name: "earth",
			// .....###...
			// .....#.#...
			// .....#.#...
			// .....#.#...
			// .###.#.####
			// .#.#.#....#
			// .#.###....#
			// .#........#
			// .##########
			input: `R 2
D 4
R 3
D 4
L 9
U 4
R 2
D 2
R 2
U 6`,
			want: 60,
		},
	}
	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			if got := part1(test.input); got != test.want {
				t.Fatalf("part1 got %d, want %d", got, test.want)
			}
			newInput := ""
			for _, line := range aocstrings.Lines(test.input) {
				if line[0] == 'R' {
					newInput += "L " + line[2:]
				} else if line[0] == 'L' {
					newInput += "R " + line[2:]
				} else {
					newInput += line
				}
				newInput += "\n"
			}
			if got := part1(newInput); got != test.want {
				t.Fatalf("part1 got %d, want %d", got, test.want)
			}
		})
	}
}

func BenchmarkPart2(b *testing.B) {
	for i := 0; i < b.N; i++ {
		want := 87716969654406
		got := part2(input)
		if got != want {
			b.Fatalf("part2 got %d, want %d", got, want)
		}
	}
}
