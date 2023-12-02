package main

import (
	_ "embed"
	"fmt"

	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

//go:embed input.txt
var input string

func isVisible(trees [][]int, i, j int) bool {
	if i == 0 || i == len(trees)-1 || j == 0 || j == len(trees[i])-1 {
		return true
	}
	t := trees[i][j]
	visible := true
	for k := 0; k < j; k++ {
		if trees[i][k] >= t {
			visible = false
			break
		}
	}
	if visible {
		return true
	}
	visible = true
	for k := len(trees[i]) - 1; k > j; k-- {
		if trees[i][k] >= t {
			visible = false
			break
		}
	}
	if visible {
		return true
	}
	visible = true
	for k := 0; k < i; k++ {
		if trees[k][j] >= t {
			visible = false
			break
		}
	}
	if visible {
		return true
	}
	visible = true
	for k := len(trees) - 1; k > i; k-- {
		if trees[k][j] >= t {
			visible = false
			break
		}
	}
	return visible
}

func parseTreeGrid(input string) [][]int {
	return aocslices.Map(aocstrings.Lines(input), func(s string) []int {
		return aocslices.Map([]byte(s), func(c byte) int {
			return int(c) - '0'
		})
	})
}

func part1(input string) int {
	trees := parseTreeGrid(input)

	visible := 0
	for i := 0; i < len(trees); i++ {
		for j := 0; j < len(trees[i]); j++ {
			if isVisible(trees, i, j) {
				visible++
			}
		}
	}
	return visible
}

func calculateScenicScore(trees [][]int, i, j int) int {
	t := trees[i][j]
	score := 1
	dist := 0
	for k := j - 1; k >= 0; k-- {
		dist++
		if trees[i][k] >= t {
			break
		}
	}
	score *= dist
	dist = 0
	for k := j + 1; k < len(trees); k++ {
		dist++
		if trees[i][k] >= t {
			break
		}
	}
	score *= dist
	dist = 0
	for k := i - 1; k >= 0; k-- {
		dist++
		if trees[k][j] >= t {
			break
		}
	}
	score *= dist
	dist = 0
	for k := i + 1; k < len(trees); k++ {
		dist++
		if trees[k][j] >= t {
			break
		}
	}
	score *= dist
	return score
}

func part2(input string) int {
	trees := parseTreeGrid(input)

	bestScore := 0
	for i := 0; i < len(trees); i++ {
		for j := 0; j < len(trees[i]); j++ {
			score := calculateScenicScore(trees, i, j)
			if score > bestScore {
				bestScore = score
			}
		}
	}
	return bestScore
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
