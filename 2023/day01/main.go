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
	input  string
	digits = []string{
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
	idx := len(line)
	if reverse {
		line = aocstrings.Reverse(line)
	}
	var val int
	if useWords {
		for d, digit := range digits {
			if reverse {
				digit = aocstrings.Reverse(digit)
			}
			i := strings.Index(line, digit)
			if i != -1 && i < idx {
				idx = i
				val = d + 1
			}
		}
	}
	if digitIdx := strings.IndexFunc(line, unicode.IsDigit); digitIdx != -1 && digitIdx < idx {
		val = int(line[digitIdx] - '0')
	}
	return val
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
