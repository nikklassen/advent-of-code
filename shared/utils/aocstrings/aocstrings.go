package aocstrings

import (
	"slices"
	"strconv"
	"strings"

	"github.com/nikklassen/advent-of-code/shared/utils"
	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
)

// Lines of the string.
func Lines(input string) []string {
	return strings.Split(strings.TrimSpace(input), "\n")
}

// Paragrahs of the string.
func Paragraphs(input string) []string {
	return strings.Split(strings.TrimSpace(input), "\n\n")
}

func TryTrimPrefix(s string, prefix string) (string, bool) {
	s2 := strings.TrimPrefix(s, prefix)
	return s2, s2 != s
}

func TryTrimSuffix(s string, suffix string) (string, bool) {
	s2 := strings.TrimSuffix(s, suffix)
	return s2, s2 != s
}

func Reverse(s string) string {
	b := []byte(s)
	slices.Reverse(b)
	return string(b)
}

func MustAtoi(s string) int {
	return utils.Must(strconv.Atoi(s))
}

func RuneGrid(s string) [][]rune {
	var grid [][]rune
	for _, row := range Lines(s) {
		grid = append(grid, []rune(row))
	}
	return grid
}

func SpaceSeparatedInts(s string) []int {
	return aocslices.Map(strings.Fields(s), MustAtoi)
}

// Set rune `i` of string `s` to `r`.
func Set(s string, i int, r rune) string {
	var ret string
	if i > 0 {
		ret = s[:i]
	}
	ret += string(r)
	if i < len(s) {
		ret += s[i+1:]
	}
	return ret
}
