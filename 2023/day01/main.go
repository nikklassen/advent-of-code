package main

import (
	_ "embed"
	"fmt"
	"strings"
	"unicode"

	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

var (
	//go:embed input.txt
	input      string
	digitWords = []string{
		"one",
		"two",
		"three",
		"four",
		"five",
		"six",
		"seven",
		"eight",
		"nine",
	}
)

func findDigit(line string, reverse, useWords bool) int {
	i := 0
	inc := 1
	if reverse {
		i = len(line) - 1
		inc = -1
	}
	for {
		if useWords {
			for d, digit := range digitWords {
				if strings.HasPrefix(line[i:], digit) {
					return d + 1
				}
			}
		}
		if unicode.IsDigit([]rune(line)[i]) {
			return int(line[i] - '0')
		}
		i += inc
	}
}

func extractValues(input string, useWords bool) []int {
	var ret []int
	for _, line := range aocstrings.Lines(input) {
		first := findDigit(line, false, useWords)
		last := findDigit(line, true, useWords)
		ret = append(ret, first*10+last)
	}
	return ret
}

func part1(input string) int {
	return aocslices.Sum(extractValues(input, false))
}

func part2(input string) int {
	return aocslices.Sum(extractValues(input, true))
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
