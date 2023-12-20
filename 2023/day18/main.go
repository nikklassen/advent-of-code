package main

import (
	_ "embed"
	"fmt"
	"math"
	"slices"
	"strconv"
	"strings"

	"github.com/nikklassen/advent-of-code/shared/grid"
	"github.com/nikklassen/advent-of-code/shared/utils"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

var (
	//go:embed input.txt
	input string
)

type inst struct {
	dir  grid.Index
	dist int
}

type pathMap struct {
	vert map[int][][]int
	hori map[int][][]int
}

func newPathMap() *pathMap {
	return &pathMap{
		vert: map[int][][]int{},
		hori: map[int][][]int{},
	}
}

func (m *pathMap) AddSegment(start, end grid.Index) {
	if start.X == end.X {
		m.vert[start.X] = append(m.vert[start.X], []int{min(start.Y, end.Y), max(start.Y, end.Y)})
	} else {
		m.hori[start.Y] = append(m.hori[start.Y], []int{min(start.X, end.X), max(start.X, end.X)})
	}
}

func (m *pathMap) Bounds() (int, int, int, int) {
	minX, minY := math.MaxInt, math.MaxInt
	maxX, maxY := math.MinInt, math.MinInt
	for x, segs := range m.vert {
		minX = min(minX, x)
		maxX = max(maxX, x)
		for _, seg := range segs {
			minY = min(seg[0], minY)
			maxY = max(seg[1], maxY)
		}
	}
	return minX, maxX, minY, maxY
}

func makeInsts(input string) []inst {
	var ret []inst
	for _, line := range aocstrings.Lines(input) {
		fields := strings.Fields(line)
		i := inst{}
		switch fields[0] {
		case "U":
			i.dir = grid.Up
		case "D":
			i.dir = grid.Down
		case "R":
			i.dir = grid.Right
		case "L":
			i.dir = grid.Left
		}
		i.dist = aocstrings.MustAtoi(fields[1])
		ret = append(ret, i)
	}
	return ret
}

func makeGrid(insts []inst) *pathMap {
	g := &pathMap{
		vert: map[int][][]int{},
		hori: map[int][][]int{},
	}
	curr := grid.I(0, 0)
	for _, inst := range insts {
		var next grid.Index
		switch inst.dir {
		case grid.Up:
			next = curr.Add(grid.I(0, -inst.dist))
		case grid.Down:
			next = curr.Add(grid.I(0, inst.dist))
		case grid.Left:
			next = curr.Add(grid.I(-inst.dist, 0))
		case grid.Right:
			next = curr.Add(grid.I(inst.dist, 0))
		}
		g.AddSegment(curr, next)
		curr = next
	}
	return g
}

type hLine struct {
	Y, Start, End int
}

func (l hLine) Len() int {
	return l.End - l.Start + 1
}

func xor(y int, a, b hLine) []hLine {
	if b.Start < a.Start {
		a, b = b, a
	}
	if a.End < b.Start {
		return []hLine{{y, a.Start, a.End}, {y, b.Start, b.End}}
	} else if a.End == b.Start {
		return []hLine{{y, a.Start, b.End}}
	}
	var ret []hLine
	if a.Start < b.Start {
		ret = append(ret, hLine{y, a.Start, b.Start})
	}
	if a.End > b.End {
		ret = append(ret, hLine{y, b.End, a.End})
	} else if a.End < b.End {
		ret = append(ret, hLine{y, a.End, b.End})
	}
	return ret
}

func xorAll(y int, as []hLine, bs []hLine) []hLine {
	var sorted []hLine
	sorted = append(sorted, as...)
	sorted = append(sorted, bs...)
	slices.SortFunc(sorted, func(a, b hLine) int {
		return a.Start - b.Start
	})
	var ret []hLine
	for len(sorted) > 1 {
		parts := xor(y, sorted[0], sorted[1])
		if len(parts) == 2 {
			ret = append(ret, parts[0])
			sorted = sorted[1:]
			sorted[0] = parts[1]
		} else if len(parts) == 1 {
			sorted = sorted[1:]
			sorted[0] = parts[0]
		} else {
			sorted = sorted[2:]
		}
	}
	ret = append(ret, sorted...)
	return ret
}

func or(y int, a, b hLine) []hLine {
	if b.Start < a.Start {
		a, b = b, a
	}
	if a.End < b.Start {
		return []hLine{{y, a.Start, a.End}, {y, b.Start, b.End}}
	}
	return []hLine{{y, a.Start, max(a.End, b.End)}}
}

func orAll(y int, as []hLine, bs []hLine) []hLine {
	var sorted []hLine
	sorted = append(sorted, as...)
	sorted = append(sorted, bs...)
	slices.SortFunc(sorted, func(a, b hLine) int {
		return a.Start - b.Start
	})

	var ret []hLine
	for len(sorted) > 1 {
		parts := or(y, sorted[0], sorted[1])
		sorted = sorted[1:]
		if len(parts) == 2 {
			ret = append(ret, parts[0])
			sorted[0] = parts[1]
		} else {
			sorted[0] = parts[0]
		}
	}
	ret = append(ret, sorted...)
	return ret
}

func lenAll(as []hLine) int {
	var ret int
	for _, a := range as {
		ret += a.Len()
	}
	return ret
}

func part1(input string) int {
	insts := makeInsts(input)
	return findArea(insts)
}

func findArea(insts []inst) int {
	g := makeGrid(insts)
	lines := []hLine{}
	for y, segs := range g.hori {
		for _, seg := range segs {
			lines = append(lines, hLine{
				Y:     y,
				Start: seg[0],
				End:   seg[1],
			})
		}
	}
	slices.SortFunc(lines, func(a, b hLine) int {
		if a.Y != b.Y {
			return a.Y - b.Y
		}
		return a.Start - b.Start
	})
	curr := []hLine{lines[0]}
	oldY := lines[0].Y
	tot := lenAll(curr)
	for i := 1; i < len(lines); i++ {
		newY := lines[i].Y
		oldLen := lenAll(curr)

		tot += oldLen * (newY - oldY - 1)

		var newLines []hLine
		for i < len(lines) && lines[i].Y == newY {
			newLines = append(newLines, lines[i])
			i++
		}
		i--

		ored := orAll(newY, newLines, curr)
		xored := xorAll(newY, newLines, curr)

		tot += lenAll(ored)

		curr = xored
		oldY = newY
	}
	return tot
}

func makeInsts2(input string) []inst {
	var ret []inst
	for _, line := range aocstrings.Lines(input) {
		idx := strings.IndexRune(line, '#')
		line = line[idx+1 : len(line)-1]
		i := inst{}
		switch line[len(line)-1] {
		case '0':
			i.dir = grid.Right
		case '1':
			i.dir = grid.Down
		case '2':
			i.dir = grid.Left
		case '3':
			i.dir = grid.Up
		}
		i.dist = int(utils.Must(strconv.ParseInt(line[:len(line)-1], 16, 64)))
		ret = append(ret, i)
	}
	return ret
}

func part2(input string) int {
	insts := makeInsts2(input)
	return findArea(insts)
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
