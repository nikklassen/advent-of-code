package main

import (
	_ "embed"
	"fmt"
	"strings"

	"github.com/nikklassen/advent-of-code/shared/utils"
	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

var (
	//go:embed input.txt
	input string
)

func decrementPattern(pattern string) string {
	if len(pattern) == 1 || pattern[1] == ',' {
		return string(pattern[0]-1) + pattern[1:]
	}
	if pattern[:2] == "10" {
		return "9" + pattern[2:]
	}
	return "1" + string(pattern[1]-1) + pattern[2:]
}

type cacheKey utils.Tuple3[string, string, bool]

var cache = map[cacheKey]int{}

func countArrangements(line string, pattern string, inPattern bool) int {
	key := cacheKey{line, pattern, inPattern}
	if v, ok := cache[key]; ok {
		return v
	}
	inner := func(line string, pattern string, inPattern bool) int {
		var skipNext bool
		if len(pattern) > 0 && pattern[0] == '0' {
			if len(pattern) == 1 {
				pattern = ""
			} else {
				pattern = pattern[2:]
			}
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
				newPattern := decrementPattern(pattern)
				tot += countArrangements(line[1:], newPattern, true)
			}
		}
		return tot
	}
	v := inner(line, pattern, inPattern)
	cache[key] = v
	return v
}

func part1(input string) int {
	lines := aocstrings.Lines(input)
	var tot int
	for _, line := range lines {
		line, pattern, _ := strings.Cut(line, " ")
		tot += countArrangements(line, pattern, false)
	}
	return tot
}

func part2(input string) int {
	lines := aocstrings.Lines(input)
	var tot int
	for _, line := range lines {
		line, pattern, _ := strings.Cut(line, " ")
		line = strings.Join(aocslices.Repeat(line, 5), "?")
		pattern = strings.Join(aocslices.Repeat(pattern, 5), ",")
		tot += countArrangements(line, pattern, false)
	}
	return tot
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
