package main

import (
	_ "embed"
	"fmt"

	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
)

var (
	//go:embed test_input.txt
	input string
)

func TODO(input string) []int {

}

func part1(input string) int {
	return aocslices.Sum(TODO(input))
}

// func part2(input string) int {
// 	return aocslices.Sum(TODO(input))
// }

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	// fmt.Printf("part 2: %d\n", part2(input))
}
