package main

import (
	_ "embed"
	"fmt"
	"slices"

	"github.com/nikklassen/advent-of-code/shared/grid"
	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

var (
	//go:embed input.txt
	input string
)

func roll(g grid.Grid[rune], dir grid.Index) {
	var cells []grid.IndexedCell[rune]
	switch dir {
	case grid.Up:
		cells = g.IndexedCells()
	case grid.Down:
		cells = g.IndexedCells()
		slices.Reverse(cells)
	case grid.Left:
		cells = g.IndexedCellsByColumn()
	case grid.Right:
		cells = g.IndexedCellsByColumn()
		slices.Reverse(cells)
	}
	for _, c := range cells {
		if c.Value != 'O' {
			continue
		}
		idx := c.Idx
		oldIdx := idx
		for {
			v, ok := g.Lookup(idx.Add(dir))
			if !ok {
				break
			}
			if v == '.' {
				idx = idx.Add(dir)
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
	roll(g, grid.Up)
	return load(g)
}

func marshalGrid(g grid.Grid[rune]) string {
	var ret string
	for _, line := range g {
		ret += string(line)
	}
	return ret
}

func part2(input string) int {
	g := grid.Grid[rune](aocstrings.RuneGrid(input))
	seen := map[string]int{}
	grids := []string{}
	var cycleStart int
outer:
	for {
		for _, dir := range []grid.Index{grid.Up, grid.Left, grid.Down, grid.Right} {
			roll(g, dir)
		}
		m := marshalGrid(g)
		if start, ok := seen[m]; ok {
			cycleStart = start
			grids = grids[cycleStart:]
			break outer
		}
		seen[m] = len(grids)
		grids = append(grids, m)
	}
	// Subtract 1 from cycle count because it's one indexed
	final := aocslices.Chunks([]rune(grids[((1_000_000_000-1)-cycleStart)%len(grids)]), g.LenCols())
	return load(final)
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
