package main

import (
	_ "embed"
	"fmt"

	"github.com/nikklassen/advent-of-code/shared/grid"
	"github.com/nikklassen/advent-of-code/shared/utils/aocmaps"
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
			switch idx.Y {
			case prev.Y + 1:
				return grid.I(idx.X, idx.Y+1), true
			case prev.Y - 1:
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
			switch idx.X {
			case prev.X + 1:
				return grid.I(idx.X+1, idx.Y), true
			case prev.X - 1:
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
	for _, ic := range g.IndexedCells() {
		if ic.Cell == 'S' {
			return ic.Idx
		}
	}
	panic("Start not found")
}

func findEnds(g grid.Grid[rune], start grid.Index) (grid.Index, grid.Index) {
	var ends []grid.Index
	for _, i := range grid.Adjacent(start) {
		if _, ok := nextTile(g, i, start); ok {
			ends = append(ends, i)
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

type pathCell struct {
	isPath bool
	c      rune
}

type floodableCell struct {
	isOrig  bool
	c       rune
	visited bool
}

func expandGrid(g grid.Grid[*pathCell]) grid.Grid[*floodableCell] {
	newG := grid.NewGridSize[*floodableCell](len(g[0])*2+1, len(g)*2+1)
	for _, idx := range newG.Indexes() {
		newG.Set(idx, &floodableCell{c: '.'})
	}
	for _, ic := range g.IndexedCells() {
		x := ic.Idx.X
		y := ic.Idx.Y
		pc := ic.Cell

		c := rune('.')
		if pc.isPath {
			c = pc.c
		}
		newX := x*2 + 1
		newY := y*2 + 1
		newG.Set(grid.I(newX, newY), &floodableCell{c: c, isOrig: true})
		right, up := '.', '.'
		switch c {
		case '-', 'L':
			right = '-'
			up = '.'
		case '|', '7':
			right = '.'
			up = '|'
		case 'F':
			right = '-'
			up = '|'
		case 'J':
			right = '.'
			up = '.'
		case 'S':
			if pc, ok := g.Lookup(grid.I(x+1, y)); ok && (pc.c == 'J' || pc.c == '7' || pc.c == '-') {
				right = '-'
			}
			if pc, ok := g.Lookup(grid.I(x, y+1)); ok && (pc.c == 'J' || pc.c == 'L' || pc.c == '|') {
				up = '|'
			}
		}
		newG.Set(grid.I(newX+1, newY), &floodableCell{c: right})
		newG.Set(grid.I(newX, newY+1), &floodableCell{c: up})
	}
	return newG
}

func walkPath(g grid.Grid[rune]) grid.Grid[*pathCell] {
	pg := grid.FromGridSize[*pathCell](g)
	for _, ic := range g.IndexedCells() {
		pg.Set(ic.Idx, &pathCell{c: ic.Cell})
	}
	start := findStart(g)
	pg.Get(start).isPath = true
	curr, _ := findEnds(g, start)
	prev := start
	for curr != start {
		pg.Get(curr).isPath = true
		next, _ := nextTile(g, curr, prev)
		prev = curr
		curr = next
	}
	return pg
}

func findInside(input string) int {
	g := grid.Grid[rune](aocstrings.RuneGrid(input))
	pathG := walkPath(g)
	expanded := expandGrid(pathG)
	toVisit := map[grid.Index]bool{grid.I(0, 0): true}
	for len(toVisit) > 0 {
		nextIdx, _, toVisit := aocmaps.Pop(toVisit)
		next := expanded.Get(nextIdx)
		next.visited = true
		for _, idx := range grid.AdjacentNoDiagonal(nextIdx) {
			if toVisit[idx] {
				continue
			}
			if e, ok := expanded.Lookup(idx); ok && !e.visited && e.c == '.' {
				toVisit[idx] = true
			}
		}
	}
	var inside int
	for _, c := range expanded.Cells() {
		if c.isOrig && c.c == '.' && !c.visited {
			inside++
		}
	}
	return inside
}

func part2(input string) int {
	return findInside(input)
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
