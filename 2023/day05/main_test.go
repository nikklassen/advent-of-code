package main

import (
	"testing"

	"github.com/google/go-cmp/cmp"
	"github.com/google/go-cmp/cmp/cmpopts"
)

func TestPart1(t *testing.T) {
	want := 240320250
	got := part1(input)
	if got != want {
		t.Errorf("part1 got %d, want %d", got, want)
	}
}

func TestMapSpanByRange(t *testing.T) {
	tests := []struct {
		name     string
		in       span
		def      span
		mapped   span
		unmapped []span
	}{
		{
			name:     "too_low",
			in:       span{1, 2},
			def:      span{3, 2},
			unmapped: []span{{1, 2}},
		},
		{
			name:     "too_high",
			in:       span{5, 1},
			def:      span{3, 2},
			unmapped: []span{{5, 1}},
		},
		{
			name:     "overlap_bottom",
			in:       span{1, 3},
			def:      span{3, 2},
			mapped:   span{3, 1},
			unmapped: []span{{1, 2}},
		},
		{
			name:     "overlap_top",
			in:       span{5, 2},
			def:      span{3, 3},
			mapped:   span{5, 1},
			unmapped: []span{{6, 1}},
		},
		{
			name:     "in_contains",
			in:       span{1, 6},
			def:      span{3, 3},
			mapped:   span{3, 3},
			unmapped: []span{{1, 2}, {6, 1}},
		},
	}
	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			gotMapped, gotUnmapped := mapSpanByRange(test.in, test.def)
			if gotMapped != test.mapped {
				t.Errorf("got mapped %v, want %v", gotMapped, test.mapped)
			}
			if !cmp.Equal(test.unmapped, gotUnmapped, cmp.AllowUnexported(span{})) {
				t.Errorf("got unmapped %v, want %v", gotUnmapped, test.unmapped)
			}
		})
	}
}

func TestMapSpanByRangeMap(t *testing.T) {
	tests := []struct {
		name string
		in   span
		rm   *rangeMap
		want []span
	}{
		{
			name: "all_outside",
			in:   span{1, 2},
			rm: &rangeMap{
				in:  []span{{3, 2}},
				out: []span{{10, 2}},
			},
			want: []span{{1, 2}},
		},
		{
			name: "overlap",
			in:   span{1, 3},
			rm: &rangeMap{
				in:  []span{{3, 2}},
				out: []span{{10, 2}},
			},
			want: []span{{1, 2}, {10, 1}},
		},
	}
	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			got := mapSpanByRangeMap(test.rm, test.in)
			if !cmp.Equal(test.want, got, cmp.AllowUnexported(span{}), cmpopts.SortSlices(func(a, b span) bool {
				return a.start < b.start
			})) {
				t.Errorf("got %v, want %v", got, test.want)
			}
		})
	}
}

func TestPart2(t *testing.T) {
	want := 28580589
	got := part2(input)
	if got != want {
		t.Errorf("part2 got %d, want %d", got, want)
	}
}
