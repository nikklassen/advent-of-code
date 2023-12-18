package main

import (
	_ "embed"
	"fmt"

	"github.com/beefsack/go-astar"
	"github.com/nikklassen/advent-of-code/shared/grid"
	"github.com/nikklassen/advent-of-code/shared/pqueue"
	"github.com/nikklassen/advent-of-code/shared/utils/aocmath"
	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

var (
	//go:embed input.txt
	input string
)

type node struct {
	idx       grid.Index
	dir       grid.Index
	runLength int
}

type pqNode struct {
	node part1Node
	loss float64
	rank float64
}

func (n pqNode) Less(other pqNode) bool {
	return n.rank < other.rank
}

func findPath(start, target part1Node) (int, bool) {
	minLoss := map[node]float64{}
	fringe := pqueue.PriorityQueue[pqNode]{}

	fringe.Push(pqNode{
		node: start,
		loss: 0,
	})
	minLoss[start.node] = 0

	visited := map[node]bool{}

	for fringe.Len() > 0 {
		currNode := fringe.Pop()
		curr := currNode.node
		currLoss := currNode.loss
		if visited[curr.node] {
			continue
		}

		visited[curr.node] = true

		if curr.idx == target.idx {
			return int(currLoss), true
		}

		for _, astarN := range curr.PathNeighbors() {
			n := astarN.(part1Node)
			if visited[n.node] {
				continue
			}
			loss := currLoss + curr.PathNeighborCost(n)
			if prev, ok := minLoss[n.node]; ok && loss > prev {
				continue
			}
			minLoss[n.node] = loss
			fringe.Push(pqNode{n, loss, loss + n.PathEstimatedCost(target)})
		}
	}
	return 0, false
}

type part1Node struct {
	g                          *grid.Grid[int]
	minRunLength, maxRunLength int
	node
}

func (curr part1Node) PathNeighbors() []astar.Pather {
	var neighbours []astar.Pather
	for _, n := range curr.g.FilterValid(grid.AdjacentNoDiagonal(curr.idx)) {
		dir := n.Sub(curr.idx)
		if dir == grid.FlipDir(curr.dir) {
			continue
		}
		if dir != curr.dir && curr.runLength < curr.minRunLength {
			continue
		}
		runLength := 1
		if dir == curr.dir {
			runLength = curr.runLength + 1
		}
		if runLength > curr.maxRunLength {
			continue
		}
		neighbours = append(neighbours, part1Node{
			g:            curr.g,
			minRunLength: curr.minRunLength,
			maxRunLength: curr.maxRunLength,
			node: node{
				idx:       n,
				dir:       dir,
				runLength: runLength,
			},
		})
	}
	return neighbours
}

func (curr part1Node) PathNeighborCost(to astar.Pather) float64 {
	other := to.(part1Node).idx
	if other == curr.idx {
		return 0
	}
	return float64(curr.g.Get(other))
}

func (curr part1Node) PathEstimatedCost(to astar.Pather) float64 {
	idx := curr.idx
	other := to.(part1Node).idx
	return float64(aocmath.Abs(other.X-idx.X) + aocmath.Abs(other.Y-idx.Y))
}

func parseGrid(input string) grid.Grid[int] {
	var g grid.Grid[int]
	for _, line := range aocstrings.Lines(input) {
		g = append(g, aocslices.Map([]byte(line), func(s byte) int {
			return int(s - '0')
		}))
	}
	return g
}

func part1(input string) int {
	g := parseGrid(input)
	dist, _ := findPath(part1Node{
		g:            &g,
		maxRunLength: 3,
		node:         node{},
	}, part1Node{
		g:            &g,
		maxRunLength: 3,
		node: node{
			idx: grid.I(g.LenCols()-1, len(g)-1),
		},
	})
	return dist
}

func part2(input string) int {
	g := parseGrid(input)
	for _, dir := range []grid.Index{grid.Right, grid.Down} {
		dist, ok := findPath(part1Node{
			g:            &g,
			minRunLength: 4,
			maxRunLength: 10,
			node: node{
				dir: dir,
			},
		}, part1Node{
			g:            &g,
			minRunLength: 4,
			maxRunLength: 10,
			node: node{
				idx: grid.I(g.LenCols()-1, len(g)-1),
			},
		})
		if ok {
			return dist
		}
	}
	panic("Failed to find target")
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
