package main

import (
	_ "embed"
	"fmt"

	"github.com/nikklassen/advent-of-code/shared/grid"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

var (
	//go:embed input.txt
	input string
)

func roll(g grid.Grid[rune]) {
	for _, c := range g.IndexedCells() {
		if c.Cell != 'O' {
			continue
		}
		idx := c.Idx
		oldIdx := idx
		for idx.Y > 0 {
			up := grid.I(idx.X, idx.Y-1)
			if g.Get(up) == '.' {
				idx = up
			} else {
				break
			}
		}
		if oldIdx != idx {
			g.Set(oldIdx, '.')
			g.Set(idx, 'O')
		}
	}
}

func load(g grid.Grid[rune]) int {
	var load int
	for col := 0; col < g.LenCols(); col++ {
		for row := 0; row < len(g); row++ {
			if g.Get(grid.I(col, row)) != 'O' {
				continue
			}
			load += len(g) - row
		}
	}
	return load
}

func part1(input string) int {
	g := aocstrings.RuneGrid(input)
	roll(g)
	return load(g)
}

// func part2(input string) int {
// 	return 0
// }

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	// fmt.Printf("part 2: %d\n", part2(input))
}
