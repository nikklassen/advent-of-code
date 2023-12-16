package main

import (
	_ "embed"
	"fmt"

	"github.com/nikklassen/advent-of-code/shared/grid"
	"github.com/nikklassen/advent-of-code/shared/utils/aocmath"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

var (
	//go:embed input.txt
	input string
)

type star struct {
	origIdx, expandedIdx grid.Index
}

func expandStars(g grid.Grid[rune], stars []*star, expansionFactor int) {
rowLoop:
	for y := 0; y < len(g); y++ {
		for x := range g[y] {
			if g.Get(grid.I(x, y)) == '#' {
				continue rowLoop
			}
		}

		for _, s := range stars {
			if s.origIdx.Y > y {
				s.expandedIdx.Y += expansionFactor
			}
		}
	}
colLoop:
	for x := 0; x < len(g[0]); x++ {
		for y := range g {
			if g.Get(grid.I(x, y)) == '#' {
				continue colLoop
			}
		}
		for _, s := range stars {
			if s.origIdx.X > x {
				s.expandedIdx.X += expansionFactor
			}
		}
	}
}

func findTotalDistance(input string, expansionFactor int) int {
	g := grid.Grid[rune](aocstrings.RuneGrid(input))
	var stars []*star
	for _, ic := range g.IndexedCells() {
		if ic.Value != '#' {
			continue
		}
		stars = append(stars, &star{
			origIdx:     ic.Idx,
			expandedIdx: ic.Idx,
		})
	}
	expandStars(g, stars, expansionFactor)
	var totalDist int
	for i, xs := range stars {
		x := xs.expandedIdx
		for _, ys := range stars[i+1:] {
			y := ys.expandedIdx
			totalDist += aocmath.Abs(x.X-y.X) + aocmath.Abs(x.Y-y.Y)
		}
	}
	return totalDist
}

func part1(input string) int {
	return findTotalDistance(input, 1)
}

func part2(input string) int {
	return findTotalDistance(input, 999_999)
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
