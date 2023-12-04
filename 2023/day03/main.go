package main

import (
	"fmt"
	"strings"
	"unicode"

	"github.com/nikklassen/advent-of-code/shared/grid"
	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"

	_ "embed"
)

var (
	//go:embed input.txt
	input string
)

func findParts(input string) int {
	parts := map[int]int{}
	chars := aocstrings.RuneGrid(input)
	g := grid.NewGridSize[string](len(chars[0]), len(chars))
	for y, row := range chars {
		for x := 0; x < len(row); x++ {
			digits := aocslices.TakeWhile(row[x:], unicode.IsDigit)
			if len(digits) == 0 {
				continue
			}
			num := aocstrings.MustAtoi(string(digits))
			val := fmt.Sprintf("%d_%d%d", num, x, y)
			for range digits {
				g.Set(grid.Index{X: x, Y: y}, val)
				x++
			}
		}
	}
	// spew.Dump(g)
	for y, row := range chars {
		for x, c := range row {
			if c == '.' || unicode.IsDigit(rune(c)) {
				continue
			}
			for _, i := range grid.Adjacent(grid.Index{X: x, Y: y}) {
				if val, ok := g.Get(i); ok && val != "" {
					n, _, _ := strings.Cut(val, "_")
					num := aocstrings.MustAtoi(n)
					parts[num] += 1
				}
			}
		}
	}
	var ret int
	for n := range parts {
		ret += n
	}
	return ret
}

func part1(input string) int {
	return findParts(input)
}

// func part2(input string) int {
// 	return aocslices.Sum(extractValues(input, true))
// }

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	// fmt.Printf("part 2: %d\n", part2(input))
}
