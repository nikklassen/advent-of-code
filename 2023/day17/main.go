package main

import (
	_ "embed"
	"fmt"
	"math"
	"slices"

	"github.com/nikklassen/advent-of-code/shared/grid"
	"github.com/nikklassen/advent-of-code/shared/pqueue"
	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

var (
	//go:embed test_input.txt
	input string
)

type node struct {
	g         *grid.Grid[int]
	idx       grid.Index
	dir       grid.Index
	loss      int
	runLength int
	path      []node
}

func (n node) Less(other node) bool {
	return n.loss < other.loss
}

func findPath(g grid.Grid[int]) int {
	minLoss := grid.FromGridSize[int, int](g)
	minLoss.Fill(math.MaxInt)
	fringe := pqueue.PriorityQueue[node]{}

	fringe.Push(node{})
	minLoss.Set(grid.Index{}, 0)

	var minPath []node
	target := grid.I(g.LenCols()-1, len(g)-1)
	visited := map[grid.Index]bool{}

outer:
	for fringe.Len() > 0 {
		curr := fringe.Pop()
		if visited[curr.idx] {
			continue
		}
		for _, n := range g.FilterValid(grid.AdjacentNoDiagonal(curr.idx)) {
			if visited[n] {
				continue
			}
			dir := n.Sub(curr.idx)
			runLength := 0
			if dir == curr.dir {
				runLength = curr.runLength + 1
			}
			if runLength == 3 {
				continue
			}
			loss := curr.loss + g.Get(n)
			if loss > minLoss.Get(n) {
				continue
			}
			minLoss.Set(n, loss)
			next := node{
				idx:       n,
				dir:       dir,
				loss:      loss,
				runLength: runLength,
			}
			path := append(slices.Clone(curr.path), next)
			if n == target {
				minPath = path
				break outer
			}
			next.path = path
			fringe.Push(next)
		}
	}
	pathSteps := map[grid.Index]node{}
	for _, p := range minPath {
		pathSteps[p.idx] = p
	}
	for y, row := range g {
		for x, e := range row {
			if grid.I(x, y) == target {
				fmt.Print("*")
			} else if p, ok := pathSteps[grid.I(x, y)]; ok {
				c := ""
				switch p.dir {
				case grid.Up:
					c = "^"
				case grid.Down:
					c = "v"
				case grid.Left:
					c = "<"
				case grid.Right:
					c = ">"
				}
				fmt.Print(c)
			} else {
				fmt.Print(e)
			}
		}
		fmt.Println()
	}
	return minLoss.Get(target)
}

func part1(input string) int {
	var g grid.Grid[int]
	for _, line := range aocstrings.Lines(input) {
		g = append(g, aocslices.Map([]byte(line), func(s byte) int {
			if s > '9' || s < '1' {
				panic(string([]byte{s}))
			}
			return int(s - '0')
		}))
	}
	return findPath(g)
}

// func part2(input string) int {
// 	return 0
// }

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	// fmt.Printf("part 2: %d\n", part2(input))
}
