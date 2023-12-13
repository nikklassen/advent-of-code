package main

import (
	_ "embed"
	"fmt"

	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

var (
	//go:embed test_input.txt
	input string
)

func part1(input string) int {
	lines := aocstrings.Lines(input)
	return 0
}

// func part2(input string) int {
// 	return 0
// }

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	// fmt.Printf("part 2: %d\n", part2(input))
}
