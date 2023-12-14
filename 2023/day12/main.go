package main

import (
	_ "embed"
	"fmt"
	"slices"
	"strings"

	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

var (
	//go:embed input.txt
	input string
)

func countArrangements(line string, pattern []int, inPattern bool) int {
	var skipNext bool
	if len(pattern) > 0 && pattern[0] == 0 {
		pattern = pattern[1:]
		inPattern = false
		skipNext = true
	}
	if line == "" {
		if len(pattern) == 0 {
			return 1
		}
		return 0
	}
	var tot int
	if line[0] == '.' || line[0] == '?' {
		if !inPattern {
			tot += countArrangements(line[1:], pattern, false)
		}
	}
	if line[0] == '#' || line[0] == '?' {
		if !skipNext && len(pattern) != 0 {
			newPattern := slices.Clone(pattern)
			newPattern[0]--
			tot += countArrangements(line[1:], newPattern, true)
		}
	}
	return tot
}

func part1(input string) int {
	lines := aocstrings.Lines(input)
	var tot int
	for _, line := range lines {
		var patternStr string
		line, patternStr, _ := strings.Cut(line, " ")
		pattern := aocstrings.SpaceSeparatedInts(strings.ReplaceAll(patternStr, ",", " "))
		tot += countArrangements(line, pattern, false)
	}
	return tot
}

// func part2(input string) int {
// 	return 0
// }

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	// fmt.Printf("part 2: %d\n", part2(input))
}
