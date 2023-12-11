package main

import (
	_ "embed"
	"fmt"

	"github.com/nikklassen/advent-of-code/shared/grid"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

var (
	//go:embed test_input.txt
	input string
)

func nextTile(g grid.Grid[rune], idx grid.Index, prev grid.Index) grid.Index {
	switch g.Get(idx) {
	case '|':
		if prev.Y < idx.Y {
			return grid.I(idx.X, idx.Y+1)
		}
		return grid.I(idx.X, idx.Y-1)
	case '-':
		if prev.X < idx.X {
			return grid.I(idx.X+1, idx.Y)
		}
		return grid.I(idx.X-1, idx.Y)
	case 'L':
		if prev.Y < idx.Y {
			return grid.I(idx.X+1, idx.Y)
		}
		return grid.I(idx.X, idx.Y-1)
	case 'J':
		if prev.Y < idx.Y {
			return grid.I(idx.X-1, idx.Y)
		}
		return grid.I(idx.X, idx.Y-1)
	case '7':
		if prev.X < idx.X {
			return grid.I(idx.X, idx.Y+1)
		}
		return grid.I(idx.X-1, idx.Y)
	case 'F':
		if prev.Y > idx.Y {
			return grid.I(idx.X+1, idx.Y)
		}
		return grid.I(idx.X, idx.Y+1)
	default:
		panic(fmt.Sprintf("unknown movement, tile %v, previous %v", g.Get(idx), prev))
	}
}

func findFurthestPoint(input string) int {
	g := grid.Grid[rune](aocstrings.RuneGrid(input))
	var start grid.Index
outer:
	for y, row := range g {
		for x, col := range row {
			if col == 'S' {
				start = grid.I(x, y)
				break outer
			}
		}
	}
	fmt.Println(start)
	var path1, path2 grid.Index
adjLoop:
	for _, i := range grid.Adjacent(start) {
		for _, j := range grid.Adjacent(start) {
			if nextTile(g, i, start) == j {
				path1, path2 = i, j
				break adjLoop
			}
		}
	}
	path1Prev, path2Prev := start, start
	steps := 1
	for {
		path1Next := nextTile(g, path1, path1Prev)
		path2Next := nextTile(g, path2, path2Prev)
		if path1Next == path2Next || path1Next == path2 && path2Next == path1 {
			return steps
		}
		steps++
		path1Prev, path2Prev = path1, path2
		path1, path2 = path1Next, path2Next
	}
}

func part1(input string) int {
	return findFurthestPoint(input)
}

// func part2(input string) int {
// 	return aocslices.Sum(TODO(input))
// }

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	// fmt.Printf("part 2: %d\n", part2(input))
}
