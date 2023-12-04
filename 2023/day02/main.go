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
	input     string
	ballCount = map[string]int{
		"red":   12,
		"green": 13,
		"blue":  14,
	}
)

func findPossibleGames(input string) []int {
	var ret []int
outer:
	for _, line := range aocstrings.Lines(input) {
		game, sets, _ := strings.Cut(line, ": ")
		for _, set := range strings.Split(sets, "; ") {
			for _, balls := range strings.Split(set, ", ") {
				count, colour, _ := strings.Cut(balls, " ")
				if utils.MustAtoi(count) > ballCount[colour] {
					continue outer
				}
			}
		}
		ret = append(ret, utils.MustAtoi(game[5:]))
	}
	return ret
}

func part1(input string) int {
	return aocslices.Sum(findPossibleGames(input))
}

func minimumSetPowers(input string) []int {
	var ret []int
	for _, line := range aocstrings.Lines(input) {
		minBalls := map[string]int{}
		_, sets, _ := strings.Cut(line, ": ")
		for _, set := range strings.Split(sets, "; ") {
			for _, balls := range strings.Split(set, ", ") {
				count, colour, _ := strings.Cut(balls, " ")
				minBalls[colour] = max(minBalls[colour], utils.MustAtoi(count))
			}
		}
		ret = append(ret, minBalls["red"]*minBalls["green"]*minBalls["blue"])
	}
	return ret
}

func part2(input string) int {
	return aocslices.Sum(minimumSetPowers(input))
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
