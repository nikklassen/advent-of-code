package main

import (
	_ "embed"
	"fmt"
	"math"

	"github.com/nikklassen/advent-of-code/shared/grid"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

//go:embed input.txt
var input string

func parseInput(input string) ([][]int, grid.Index, grid.Index) {
	lines := aocstrings.Lines(input)
	var ret [][]int
	var start grid.Index
	var end grid.Index
	for y, line := range lines {
		row := make([]int, len(line))
		for x, c := range line {
			switch c {
			case 'S':
				start = grid.Index{X: x, Y: y}
				c = 'a'
			case 'E':
				end = grid.Index{X: x, Y: y}
				c = 'z'
			}
			row[x] = int(c) - 'a'
		}
		ret = append(ret, row)
	}
	return ret, start, end
}

func neighbours(pos grid.Index, g grid.Grid[int], climbDown bool) []grid.Index {
	currHeight := g.Get(pos)
	var allowedMove func(height int) bool
	if climbDown {
		allowedMove = func(height int) bool {
			return height <= currHeight+1
		}
	} else {
		allowedMove = func(height int) bool {
			return height >= currHeight-1
		}
	}

	var neighbours []grid.Index

	up := pos.Add(grid.Index{Y: 1})
	if upHeight, ok := g.Lookup(up); ok && allowedMove(upHeight) {
		neighbours = append(neighbours, up)
	}
	down := pos.Add(grid.Index{Y: -1})
	if downHeight, ok := g.Lookup(down); ok && allowedMove(downHeight) {
		neighbours = append(neighbours, down)
	}
	left := pos.Add(grid.Index{X: -1})
	if leftHeight, ok := g.Lookup(left); ok && allowedMove(leftHeight) {
		neighbours = append(neighbours, left)
	}
	right := pos.Add(grid.Index{X: 1})
	if rightHeight, ok := g.Lookup(right); ok && allowedMove(rightHeight) {
		neighbours = append(neighbours, right)
	}

	return neighbours
}

type state struct {
	pos   grid.Index
	steps int
}

func findEnd(start, end grid.Index, g grid.Grid[int]) int {
	fringe := []state{{start, 0}}
	visited := map[grid.Index]bool{}
	for len(fringe) > 0 {
		var s state
		s, fringe = fringe[0], fringe[1:]
		if visited[s.pos] {
			continue
		}
		visited[s.pos] = true
		for _, n := range neighbours(s.pos, g, true) {
			if n == end {
				return s.steps + 1
			}
			fringe = append(fringe, state{n, s.steps + 1})
		}
	}
	return -1
}

func part1(input string) int {
	g, start, end := parseInput(input)
	return findEnd(start, end, g)
}

func findAllStarts(g grid.Grid[int], end grid.Index) int {
	stepsG := grid.Grid[int]{}
	for _, row := range g {
		newRow := make([]int, len(row))
		stepsG = append(stepsG, newRow)
		for x := range row {
			newRow[x] = 9999
		}
	}

	fringe := []state{{end, 0}}
	visited := map[grid.Index]bool{}
	for len(fringe) > 0 {
		var s state
		s, fringe = fringe[0], fringe[1:]
		if visited[s.pos] {
			continue
		}
		visited[s.pos] = true
		for _, n := range neighbours(s.pos, g, false) {
			if v := stepsG.Get(n); s.steps+1 < v {
				stepsG.Set(n, s.steps+1)
			}
			fringe = append(fringe, state{n, s.steps + 1})
		}
	}

	min := math.MaxInt
	for y, row := range g {
		for x, v := range row {
			if v != 0 {
				continue
			}
			steps := stepsG.Get(grid.Index{X: x, Y: y})
			if steps < min {
				min = steps
			}
		}
	}
	return min
}

func part2(input string) int {
	g, _, end := parseInput(input)
	return findAllStarts(g, end)
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
