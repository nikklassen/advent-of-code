package main

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"

	"github.com/nikklassen/advent-of-code/2022/grid"
	"github.com/nikklassen/advent-of-code/2022/utils"
)

type dir int

const (
	up dir = iota + 1
	down
	right
	left
)

var (
	upDir    = grid.Index{X: 0, Y: 1}
	downDir  = grid.Index{X: 0, Y: -1}
	rightDir = grid.Index{X: 1, Y: 0}
	leftDir  = grid.Index{X: -1, Y: 0}

	dirMap = map[dir]grid.Index{
		up:    upDir,
		down:  downDir,
		right: rightDir,
		left:  leftDir,
	}
)

//go:embed input.txt
var input string

func moveKnot(head, tail grid.Index) grid.Index {
	dist := head.Sub(tail)
	absX := utils.Abs(dist.X)
	absY := utils.Abs(dist.Y)
	if absX <= 1 && absY <= 1 {
		return tail
	}
	if absX == 0 {
		if head.Y > tail.Y {
			return tail.Add(upDir)
		}
		return tail.Add(downDir)
	}
	if absY == 0 {
		if head.X > tail.X {
			return tail.Add(rightDir)
		}
		return tail.Add(leftDir)
	}
	return tail.Add(grid.Index{
		X: utils.Sign(dist.X),
		Y: utils.Sign(dist.Y),
	})
}

func moveRope(input string, length int) int {
	rope := make([]grid.Index, length)
	visited := map[grid.Index]bool{rope[length-1]: true}
	for _, s := range utils.Lines(input) {
		dirString, dist, _ := strings.Cut(s, " ")
		var d dir
		switch dirString {
		case "U":
			d = up
		case "D":
			d = down
		case "L":
			d = left
		case "R":
			d = right
		}
		count := utils.Must(strconv.Atoi(dist))

		for i := 0; i < count; i++ {
			rope[0] = rope[0].Add(dirMap[d])
			for i := 1; i < length; i++ {
				newKnot := moveKnot(rope[i-1], rope[i])
				if rope[i] == newKnot {
					break
				}
				rope[i] = newKnot
			}
			visited[rope[length-1]] = true
		}
	}
	return len(visited)
}

func part1(input string) int {
	return moveRope(input, 2)
}

func printRope(rope []grid.Index) {
	maxX, minX, maxY, minY := 0, 0, 0, 0
	for _, k := range rope {
		maxX = utils.Max(maxX, k.X)
		minX = utils.Min(minX, k.X)
		maxY = utils.Max(maxY, k.Y)
		minY = utils.Min(minY, k.Y)
	}
	var grid [][]byte
	for y := minY - 1; y < maxY+2; y++ {
		grid = append(grid, make([]byte, maxX-minX+3))
		for x := minX - 1; x < maxX+2; x++ {
			if x == 0 && y == 0 {
				grid[y-(minY-1)][x-(minX-1)] = 's'
			} else {
				grid[y-(minY-1)][x-(minX-1)] = '.'
			}
		}
	}
	for i := len(rope) - 1; i >= 0; i-- {
		k := rope[i]
		if i == 0 {
			grid[k.Y-(minY-1)][k.X-(minX-1)] = 'H'
		} else {
			grid[k.Y-(minY-1)][k.X-(minX-1)] = byte(i) + '0'
		}
	}
	for i := len(grid) - 1; i >= 0; i-- {
		fmt.Println(string(grid[i]))
	}
}

func part2(input string) int {
	return moveRope(input, 10)
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
