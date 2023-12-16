package main

import (
	_ "embed"
	"fmt"
	"slices"

	"github.com/nikklassen/advent-of-code/shared/grid"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

var (
	//go:embed input.txt
	input string
)

type beam struct {
	idx grid.Index
	dir grid.Index
}

func moveBeam(g grid.Grid[rune], b beam) []beam {
	next := b.idx.Add(b.dir)
	v, ok := g.Lookup(next)
	if !ok {
		return nil
	}
	nextDir := b.dir
	switch v {
	case '.':
		// no-op
	case '|':
		if b.dir != grid.Up && b.dir != grid.Down {
			return []beam{{next, grid.Up}, {next, grid.Down}}
		}
	case '-':
		if b.dir != grid.Left && b.dir != grid.Right {
			return []beam{{next, grid.Left}, {next, grid.Right}}
		}
	case '/':
		switch b.dir {
		case grid.Up:
			nextDir = grid.Right
		case grid.Right:
			nextDir = grid.Up
		case grid.Down:
			nextDir = grid.Left
		case grid.Left:
			nextDir = grid.Down
		}
	case '\\':
		switch b.dir {
		case grid.Up:
			nextDir = grid.Left
		case grid.Right:
			nextDir = grid.Down
		case grid.Down:
			nextDir = grid.Right
		case grid.Left:
			nextDir = grid.Up
		}
	}
	return []beam{{next, nextDir}}
}

func energize(g grid.Grid[rune], start beam) int {
	moving := []beam{start}
	visited := grid.FromGridSize[[]beam](g)
	for len(moving) > 0 {
		var nextMoving []beam
		for _, b := range moving {
			existing, ok := visited.Lookup(b.idx)
			if !ok {
				continue
			}
			if slices.ContainsFunc(existing, func(other beam) bool {
				return b == other
			}) {
				continue
			}
			visited.Set(b.idx, append(existing, b))
			nextMoving = append(nextMoving, moveBeam(g, b)...)
		}
		moving = nextMoving
	}
	var ret int
	for _, c := range visited.Cells() {
		if len(c) > 0 {
			ret++
		}
	}
	return ret
}

func part1(input string) int {
	g := grid.Grid[rune](aocstrings.RuneGrid(input))
	return energize(g, beam{dir: grid.Right})
}

func part2(input string) int {
	g := grid.Grid[rune](aocstrings.RuneGrid(input))
	m := 0
	for _, c := range g.IndexedCells() {
		if c.Idx.X == 0 {
			m = max(m, energize(g, beam{idx: c.Idx, dir: grid.Right}))
		}
		if c.Idx.X == g.LenCols() {
			m = max(m, energize(g, beam{idx: c.Idx, dir: grid.Left}))
		}
		if c.Idx.Y == 0 {
			m = max(m, energize(g, beam{idx: c.Idx, dir: grid.Down}))
		}
		if c.Idx.Y == len(g) {
			m = max(m, energize(g, beam{idx: c.Idx, dir: grid.Up}))
		}
	}
	return m
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
