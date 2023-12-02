package main

import "testing"

func TestFindDigitWord(t *testing.T) {
	d := findDigit("xfivex", false, true)
	if d != 5 {
		t.Errorf("non-reverse got %d, want %d", d, 5)
	}
	d = findDigit("xfivex", true, true)
	if d != 5 {
		t.Errorf("reverse got %d, want %d", d, 5)
	}
}
