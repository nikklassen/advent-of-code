package utils

import (
	"testing"

	"github.com/google/go-cmp/cmp"
)

func TestRangeIntersect(t *testing.T) {
	a, b := Range{End: 10}, Range{Start: 5, End: 20}
	want := Range{Start: 5, End: 10}
	got := a.Intersect(b)
	if got != want {
		t.Errorf("%v.Intersect(%v) got %v, want %v", a, b, got, want)
	}
}

func TestRangeSetIntersect(t *testing.T) {
	a := RangeSet{{0, 3}, {4, 6}}
	b := RangeSet{{2, 4}, {5, 8}}
	want := RangeSet{{2, 3}, {5, 6}}
	got := a.Intersect(b)
	if !cmp.Equal(got, want) {
		t.Errorf("%v.Intersect(%v) got %v, want %v", a, b, got, want)
	}
}
