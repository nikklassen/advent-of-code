package aocmath

import "testing"

func TestLCMAll(t *testing.T) {
	if got := LCMAll([]uint{75, 100}); got != 300 {
		t.Errorf("got %d, want %d", got, 300)
	}
}
