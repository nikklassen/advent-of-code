package main

import (
	_ "embed"
	"fmt"
	"regexp"
	"slices"
	"strings"

	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

var (
	//go:embed input.txt
	input string

	wsRegexp = regexp.MustCompile(`\s+`)
)

func numStringToNums(numString string) []int {
	numString = strings.TrimSpace(numString)
	return aocslices.Map(wsRegexp.Split(numString, -1), aocstrings.MustAtoi)
}

func matches(line string) int {
	_, line, _ = strings.Cut(line, ": ")
	winning, card, _ := strings.Cut(line, " | ")
	winningNums := numStringToNums(winning)
	cardNums := numStringToNums(card)
	matches := 0
	for _, cn := range cardNums {
		if slices.Contains(winningNums, cn) {
			matches++
		}
	}
	return matches
}

func countPoints(input string) []int {
	var points []int
	for _, line := range aocstrings.Lines(input) {
		matches := matches(line)
		if matches > 0 {
			points = append(points, 1<<(matches-1))
		}
	}
	return points
}

func part1(input string) int {
	return aocslices.Sum(countPoints(input))
}

func countCopies(input string) []int {
	lines := aocstrings.Lines(input)
	copies := make([]int, len(lines))
	for i := 0; i < len(copies); i++ {
		copies[i] = 1
	}
	for i := 0; i < len(lines); i++ {
		matches := matches(lines[i])
		for j := 0; j < matches; j++ {
			copies[i+j+1] += copies[i]
		}
	}
	return copies
}

func part2(input string) int {
	return aocslices.Sum(countCopies(input))
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
