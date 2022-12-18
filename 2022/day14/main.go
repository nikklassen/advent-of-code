package main

import (
	_ "embed"
	"fmt"
	"strings"

	"github.com/nikklassen/advent-of-code/2022/grid"
	"github.com/nikklassen/advent-of-code/2022/utils"
	"github.com/nikklassen/advent-of-code/2022/utils/aocmath"
	"github.com/nikklassen/advent-of-code/2022/utils/aocslices"
	"github.com/nikklassen/advent-of-code/2022/utils/aocstrings"
)

//go:embed input.txt
var input string

func parseRock(line string) []grid.Index {
	parts := strings.Split(line, " -> ")
	var points []grid.Index
	for _, part := range parts {
		var p grid.Index
		utils.Must(fmt.Sscanf(part, "%d,%d", &p.X, &p.Y))
		points = append(points, p)
	}
	return points
}

func findAbyss(rocks [][]grid.Index) int {
	max := 0
	for _, rock := range rocks {
		for _, p := range rock {
			if p.Y > max {
				max = p.Y
			}
		}
	}
	return max
}

func fillRockMap(rocks [][]grid.Index) map[grid.Index]bool {
	m := map[grid.Index]bool{}
	for _, rock := range rocks {
		for i := 1; i < len(rock); i++ {
			r0 := rock[i-1]
			r1 := rock[i]
			if r0.X == r1.X {
				min, max := aocmath.Min(r0.Y, r1.Y), aocmath.Max(r0.Y, r1.Y)
				for j := min; j <= max; j++ {
					m[grid.Index{X: r0.X, Y: j}] = true
				}
			} else {
				min, max := aocmath.Min(r0.X, r1.X), aocmath.Max(r0.X, r1.X)
				for j := min; j <= max; j++ {
					m[grid.Index{X: j, Y: r0.Y}] = true
				}
			}
		}
	}
	return m
}

func dropGrains(input string, hasFloor bool) int {
	rocks := aocslices.Map(aocstrings.Lines(input), parseRock)
	abyss := findAbyss(rocks)
	rockMap := fillRockMap(rocks)
	floor := abyss + 2

	grains := map[grid.Index]bool{}
	directions := []grid.Index{{Y: 1}, {X: -1, Y: 1}, {X: 1, Y: 1}}
dropGrain:
	for {
		currGrain := grid.Index{X: 500}
	fall:
		for {
			if !hasFloor && currGrain.Y >= abyss+1 {
				break dropGrain
			}
			for _, v := range directions {
				nextPos := currGrain.Add(v)
				if grains[nextPos] || rockMap[nextPos] || hasFloor && nextPos.Y == floor {
					continue
				}
				currGrain = nextPos
				continue fall
			}
			break
		}
		grains[currGrain] = true
		if hasFloor && currGrain.Y == 0 {
			break
		}
	}
	return len(grains)
}

func part1(input string) int {
	return dropGrains(input, false)
}

func part2(input string) int {
	return dropGrains(input, true)
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
