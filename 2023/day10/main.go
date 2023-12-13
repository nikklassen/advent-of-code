package main

import (
	_ "embed"
	"fmt"

	"github.com/nikklassen/advent-of-code/shared/grid"
	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

var (
	//go:embed input.txt
	input string
)

func nextTile(g grid.Grid[rune], idx grid.Index, prev grid.Index) (grid.Index, bool) {
	c, ok := g.Lookup(idx)
	if !ok {
		return grid.Index{}, false
	}
	switch {
	case idx.X == prev.X:
		switch c {
		case '|':
			if idx.Y == prev.Y+1 {
				return grid.I(idx.X, idx.Y+1), true
			} else if idx.Y == prev.Y-1 {
				return grid.I(idx.X, idx.Y-1), true
			}
		case '7':
			if idx.Y == prev.Y-1 {
				return grid.I(idx.X-1, idx.Y), true
			}
		case 'F':
			if idx.Y == prev.Y-1 {
				return grid.I(idx.X+1, idx.Y), true
			}
		case 'J':
			if idx.Y == prev.Y+1 {
				return grid.I(idx.X-1, idx.Y), true
			}
		case 'L':
			if idx.Y == prev.Y+1 {
				return grid.I(idx.X+1, idx.Y), true
			}
		}
	case idx.Y == prev.Y:
		switch c {
		case '-':
			if idx.X == prev.X+1 {
				return grid.I(idx.X+1, idx.Y), true
			} else if idx.X == prev.X-1 {
				return grid.I(idx.X-1, idx.Y), true
			}
		case '7':
			if idx.X == prev.X+1 {
				return grid.I(idx.X, idx.Y+1), true
			}
		case 'F':
			if idx.X == prev.X-1 {
				return grid.I(idx.X, idx.Y+1), true
			}
		case 'J':
			if idx.X == prev.X+1 {
				return grid.I(idx.X, idx.Y-1), true
			}
		case 'L':
			if idx.X == prev.X-1 {
				return grid.I(idx.X, idx.Y-1), true
			}
		}
	}
	return grid.Index{}, false
}

func findStart(g grid.Grid[rune]) grid.Index {
	for y, row := range g {
		for x, col := range row {
			if col == 'S' {
				return grid.I(x, y)
			}
		}
	}
	panic("Start not found")
}

func findEnds(g grid.Grid[rune], start grid.Index) (grid.Index, grid.Index) {
	var ends []grid.Index
	for _, i := range grid.Adjacent(start) {
		for _, j := range grid.Adjacent(i) {
			if next, ok := nextTile(g, i, j); ok && next == start {
				ends = append(ends, i)
				break
			}
		}
	}
	return ends[0], ends[1]
}

func findFurthestPoint(input string) int {
	g := grid.Grid[rune](aocstrings.RuneGrid(input))
	start := findStart(g)
	path1, path2 := findEnds(g, start)
	path1Prev, path2Prev := start, start
	steps := 1
	for {
		path1Next, _ := nextTile(g, path1, path1Prev)
		path2Next, _ := nextTile(g, path2, path2Prev)
		if path1Next == path2Next {
			return steps + 1
		} else if path1Next == path2 && path2Next == path1 {
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

func expandGrid(g grid.Grid[rune]) grid.Grid[rune] {
	newG := grid.NewGridSize[rune](len(g[0])*2+1, len(g)*2+1)
	for _, x := range newG[0] {
	}
}

func findInside(input string) int {
	g := grid.Grid[rune](aocstrings.RuneGrid(input))
	start := findStart(g)
	path1, _ := findEnds(g, start)
	inside := map[grid.Index]bool{}
	path1Prev := start
	for {
		path1Next, _ := nextTile(g, path1, path1Prev)
		if path1Next == start {
			break
		}
		for _, idx := range drawRays(g, path1Next) {
			inside[idx] = true
		}
		path1 = path1Next
	}
	return len(inside)
}

func part2(input string) int {
	return findInside(input)
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	// fmt.Printf("part 2: %d\n", part2(input))
}
