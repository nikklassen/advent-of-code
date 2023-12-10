package main

import (
	_ "embed"
	"fmt"
	"strings"

	"github.com/nikklassen/advent-of-code/shared/utils"
	"github.com/nikklassen/advent-of-code/shared/utils/aocmath"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

var (
	//go:embed input.txt
	input string
)

type dirMap map[string]utils.Tuple[string, string]

type circularList struct {
	idx   int
	elems []rune
}

func (cl *circularList) Reset() {
	cl.idx = 0
}

func (cl *circularList) Next() rune {
	e := cl.elems[cl.idx]
	cl.idx = (cl.idx + 1) % len(cl.elems)
	return e
}

func parseInput(input string) (*circularList, dirMap) {
	lines := aocstrings.Lines(input)
	cl := &circularList{elems: []rune(lines[0])}
	dirs := dirMap{}
	for _, line := range lines[2:] {
		start, opts, _ := strings.Cut(line, " = ")
		opts = opts[1 : len(opts)-1]
		l, r, _ := strings.Cut(opts, ", ")
		dirs[start] = utils.Tuple[string, string]{
			Item1: l,
			Item2: r,
		}
	}
	return cl, dirs
}

func part1(input string) int {
	cl, dirs := parseInput(input)
	steps := 0
	pos := "AAA"
	for pos != "ZZZ" {
		steps++
		opts := dirs[pos]
		if cl.Next() == 'L' {
			pos = opts.Item1
		} else {
			pos = opts.Item2
		}
	}
	return steps
}

type position struct {
	instrIdx int
	symbol   string
}

func part2(input string) int {
	cl, dirs := parseInput(input)
	var starts []string
	for s := range dirs {
		if s[2] == 'A' {
			starts = append(starts, s)
		}
	}
	var lengths []uint
	for _, start := range starts {
		seen := map[position]uint{}
		var steps uint
		pos := position{instrIdx: cl.idx, symbol: start}
		for {
			if lastSteps, ok := seen[pos]; ok {
				lengths = append(lengths, steps-lastSteps)
				break
			}
			seen[pos] = steps
			steps++
			opts := dirs[pos.symbol]
			pos = position{instrIdx: cl.idx}
			if cl.Next() == 'L' {
				pos.symbol = opts.Item1
			} else {
				pos.symbol = opts.Item2
			}
		}
	}
	return int(aocmath.LCMAll(lengths))
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
