package main

import (
	"fmt"
	"unicode"

	"github.com/nikklassen/advent-of-code/shared/grid"
	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
	"golang.org/x/exp/maps"

	_ "embed"
)

var (
	//go:embed input.txt
	input string
)

type gridNum struct {
	num   int
	start grid.Index
}

func buildGrid(chars [][]rune) grid.Grid[gridNum] {
	g := grid.NewGridSize[gridNum](len(chars[0]), len(chars))
	for y, row := range chars {
		for x := 0; x < len(row); x++ {
			digits := aocslices.TakeWhile(row[x:], unicode.IsDigit)
			if len(digits) == 0 {
				continue
			}
			num := aocstrings.MustAtoi(string(digits))
			a := gridNum{num: num, start: grid.I(x, y)}
			for range digits {
				g.Set(grid.Index{X: x, Y: y}, a)
				x++
			}
		}
	}
	return g
}

func addAdjacentParts(g grid.Grid[gridNum], x, y int, parts map[grid.Index]int) {
	for _, i := range grid.Adjacent(grid.I(x, y)) {
		if a := g.Get(i); a.num != 0 {
			parts[a.start] = a.num
		}
	}
}

func findParts(input string) int {
	chars := aocstrings.RuneGrid(input)
	g := buildGrid(chars)
	parts := map[grid.Index]int{}
	for y, row := range chars {
		for x, c := range row {
			if c == '.' || unicode.IsDigit(rune(c)) {
				continue
			}
			addAdjacentParts(g, x, y, parts)
		}
	}
	return aocslices.Sum(maps.Values(parts))
}

func part1(input string) int {
	return findParts(input)
}

func findGears(input string) int {
	chars := aocstrings.RuneGrid(input)
	g := buildGrid(chars)
	var ret int
	for y, row := range chars {
		for x, c := range row {
			if c != '*' {
				continue
			}
			adjacentParts := map[grid.Index]int{}
			addAdjacentParts(g, x, y, adjacentParts)
			if len(adjacentParts) == 2 {
				ret += aocslices.Product(maps.Values(adjacentParts))
			}
		}
	}
	return ret
}

func part2(input string) int {
	return findGears(input)
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
