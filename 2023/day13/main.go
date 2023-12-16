package main

import (
	_ "embed"
	"fmt"
	"os"
	"strings"

	"github.com/nikklassen/advent-of-code/shared/grid"
	"github.com/nikklassen/advent-of-code/shared/utils/aocmaps"
	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
	"golang.org/x/exp/maps"
)

var (
	//go:embed input.txt
	input string
)

func findVertLOR(lines []string) map[int]bool {
	ret := map[int]bool{}
outer:
	for x := 1; x < len(lines[0]); x++ {
		for _, line := range lines {
			left, right := line[:x], line[x:]
			left = aocstrings.Reverse(left)
			if !strings.HasPrefix(right, left) && !strings.HasPrefix(left, right) {
				continue outer
			}
		}
		ret[x] = true
	}
	return ret
}

func transpose(lines []string) []string {
	var g grid.Grid[rune]
	for _, line := range lines {
		g = append(g, []rune(line))
	}
	transposed := g.Transpose()
	var ret []string
	for _, row := range transposed {
		ret = append(ret, string(row))
	}
	return ret
}

type lor struct {
	vert, hor map[int]bool
}

func (lor lor) String() string {
	return fmt.Sprintf("vert: %v, hor: %v", lor.vert, lor.hor)
}

func (lor lor) value() int {
	return aocslices.Sum(maps.Keys(lor.vert)) + 100*aocslices.Sum(maps.Keys(lor.hor))
}

func (lor lor) equals(other lor) bool {
	return aocmaps.Equals(lor.vert, other.vert) && aocmaps.Equals(lor.hor, other.hor)
}

func minus(a, b []int) []int {
	var ret []int
outer:
	for _, x := range a {
		for _, y := range b {
			if x == y {
				continue outer
			}
		}
		ret = append(ret, x)
	}
	return ret
}

func (l lor) difference(other lor) lor {
	return lor{
		vert: aocmaps.Difference(l.vert, other.vert),
		hor:  aocmaps.Difference(l.hor, other.hor),
	}
}

func (lor lor) valid() bool {
	return len(lor.vert) > 0 || len(lor.hor) > 0
}

func findLOR(lines []string) lor {
	return lor{
		vert: findVertLOR(lines),
		hor:  findVertLOR(transpose(lines)),
	}
}

func do(input string, hasSmudges bool) int {
	chunks := aocslices.Map(aocstrings.Paragraphs(input), aocstrings.Lines)
	var tot int
	for i, chunk := range chunks {
		lor := findLOR(chunk)
		if !lor.valid() {
			fmt.Printf("didn't find line of reflection for grid %d\n", i)
			fmt.Println(strings.Join(chunk, "\n"))
			os.Exit(1)
		}
		if hasSmudges {
			lor = findLORWithSmudges(chunk, lor)
			if !lor.valid() {
				fmt.Printf("didn't find line of reflection for grid %d\n", i)
				fmt.Println(strings.Join(chunk, "\n"))
				os.Exit(1)
			}
		}
		tot += lor.value()
	}
	return tot
}

func findLORWithSmudges(lines []string, unsmudged lor) lor {
	rowLen := len(lines[0])
	for i := 0; i < (len(lines) * rowLen); i++ {
		row := i / rowLen
		col := i % rowLen
		oldRow := lines[row]
		var newRow string
		if oldRow[col] == '.' {
			newRow = aocstrings.Set(oldRow, col, '#')
		} else {
			newRow = aocstrings.Set(oldRow, col, '.')
		}
		lines[row] = newRow
		lor := findLOR(lines)
		if lor.valid() && !unsmudged.equals(lor) {
			return lor.difference(unsmudged)
		}
		lines[row] = oldRow
	}
	return lor{}
}

func part1(input string) int {
	return do(input, false)
}

func part2(input string) int {
	return do(input, true)
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
