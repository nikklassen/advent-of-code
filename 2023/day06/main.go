package main

import (
	_ "embed"
	"fmt"
	"math"
	"strings"

	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

var (
	//go:embed input.txt
	input string
)

func winningTimes(time, distance int) int {
	a := -1.0
	b := float64(time)
	c := -1.0 * float64(distance)
	part1 := (-b / (2 * a))
	part2 := math.Sqrt(b*b-4*a*c) / (2 * a)
	start := part1 + part2
	end := part1 - part2
	return int(math.Floor(end-math.SmallestNonzeroFloat64)-math.Ceil(start+math.SmallestNonzeroFloat64)) + 1
}

func computeWins(input string) int {
	lines := aocstrings.Lines(input)
	times := aocstrings.SpaceSeparatedInts(strings.TrimSpace(strings.TrimPrefix(lines[0], "Time:")))
	dists := aocstrings.SpaceSeparatedInts(strings.TrimSpace(strings.TrimPrefix(lines[1], "Distance:")))
	var winOptions []int
	for i := 0; i < len(times); i++ {
		count := winningTimes(times[i], dists[i])
		winOptions = append(winOptions, count)
	}
	return aocslices.Product(winOptions)
}

func part1(input string) int {
	return computeWins(input)
}

func part2(input string) int {
	lines := aocstrings.Lines(input)
	times := aocstrings.MustAtoi(strings.ReplaceAll(strings.TrimPrefix(lines[0], "Time:"), " ", ""))
	dists := aocstrings.MustAtoi(strings.ReplaceAll(strings.TrimPrefix(lines[1], "Distance:"), " ", ""))
	return winningTimes(times, dists)
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
