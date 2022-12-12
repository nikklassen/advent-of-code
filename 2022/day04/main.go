package main

import (
	_ "embed"
	"fmt"

	"github.com/nikklassen/advent-of-code/2022/utils"
	"github.com/nikklassen/advent-of-code/2022/utils/aocslices"
	"github.com/nikklassen/advent-of-code/2022/utils/aocstrings"
)

//go:embed input.txt
var input string

type Range utils.Tuple[int, int]
type Assignments utils.Tuple[Range, Range]

func parseAssignment(line string) Assignments {
	a := Assignments{}
	utils.Must(fmt.Sscanf(line, "%d-%d,%d-%d", &a.Item1.Item1, &a.Item1.Item2, &a.Item2.Item1, &a.Item2.Item2))
	return a
}

func fullyContained(a Assignments) bool {
	return a.Item1.Item1 >= a.Item2.Item1 && a.Item1.Item2 <= a.Item2.Item2 ||
		a.Item2.Item1 >= a.Item1.Item1 && a.Item2.Item2 <= a.Item1.Item2
}

func part1(input string) int {
	return aocslices.CountFunc(aocslices.Map(aocstrings.Lines(input), parseAssignment), fullyContained)
}

func overlap(a Assignments) bool {
	return a.Item1.Item1 >= a.Item2.Item1 && a.Item1.Item1 <= a.Item2.Item2 ||
		a.Item1.Item2 >= a.Item2.Item1 && a.Item1.Item2 <= a.Item2.Item2 ||
		a.Item2.Item1 >= a.Item1.Item1 && a.Item2.Item1 <= a.Item1.Item2 ||
		a.Item2.Item2 >= a.Item1.Item1 && a.Item2.Item2 <= a.Item1.Item2
}

func part2(input string) int {
	return aocslices.CountFunc(aocslices.Map(aocstrings.Lines(input), parseAssignment), overlap)
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
