package main

import (
	_ "embed"
	"fmt"
	"strconv"

	"github.com/nikklassen/advent-of-code/2022/utils"
	"golang.org/x/exp/slices"
)

//go:embed input.txt
var input string

func noop(line string) string {
	return line
}

func computeSumsAsc(input string) []int {
	var sums []int
	var sum int
	for _, line := range utils.MapLines(input, noop) {
		if line == "" {
			sums = append(sums, sum)
			sum = 0
			continue
		}
		sum += utils.Must(strconv.Atoi(line))
	}
	slices.Sort(sums)
	return sums
}

func part1(input string) int {
	sums := computeSumsAsc(input)
	return sums[len(sums)-1]
}

func part2(input string) int {
	sums := computeSumsAsc(input)

	return utils.Sum(sums[len(sums)-3:])
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
