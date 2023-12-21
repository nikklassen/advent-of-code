package utils

import "testing"

func TestRangeIntersect(t *testing.T) {
	a, b := Range{End: 10}, Range{Start: 5, End: 20}
	want := Range{Start: 5, End: 10}
	got := a.Intersect(b)
	if got != want {
		t.Errorf("%v.Intersect(%v) got %v, want %v", a, b, got, want)
	}
}
