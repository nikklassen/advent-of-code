package main

import (
	_ "embed"
	"fmt"

	"github.com/nikklassen/advent-of-code/2022/utils"
	"github.com/nikklassen/advent-of-code/2022/utils/aocstrings"
)

//go:embed input.txt
var input string

type Range utils.Tuple[int, int]
type Assignments utils.Tuple[Range, Range]

type inputParser struct {
	parseMoves bool
	moves      []move
	stacks     [][]byte
}

type move struct {
	count, start, end int
}

func (p *inputParser) Parse(line string) {
	if p.parseMoves {
		var m move
		fmt.Sscanf(line, "move %d from %d to %d", &m.count, &m.start, &m.end)
		m.start -= 1
		m.end -= 1
		p.moves = append(p.moves, m)
		return
	}
	if line == "" {
		p.parseMoves = true
		return
	}
	if line[1] == '1' {
		return
	}
	if len(p.stacks) == 0 {
		p.stacks = make([][]byte, (len(line)+1)/4)
	}
	for i := 0; i < len(line); i += 4 {
		if line[i+1] == ' ' {
			continue
		}
		p.stacks[i/4] = append([]byte{line[i+1]}, p.stacks[i/4]...)
	}
}

func moveBoxes(stacks [][]byte, start, end, count int) {
	startCol := stacks[start]
	toMoveStart := len(startCol) - count
	var toMove []byte
	stacks[start], toMove = startCol[:toMoveStart], startCol[toMoveStart:]
	stacks[end] = append(stacks[end], toMove...)
}

func tops(stacks [][]byte) string {
	var tops []byte
	for _, s := range stacks {
		if len(s) > 0 {
			tops = append(tops, s[len(s)-1])
		}
	}
	return string(tops)
}

func part1(input string) string {
	p := &inputParser{}
	for _, line := range aocstrings.Lines(input) {
		p.Parse(line)
	}
	for _, m := range p.moves {
		for i := 0; i < m.count; i++ {
			moveBoxes(p.stacks, m.start, m.end, 1)
		}
	}
	return tops(p.stacks)
}

func part2(input string) string {
	p := &inputParser{}
	for _, line := range aocstrings.Lines(input) {
		p.Parse(line)
	}
	for _, m := range p.moves {
		moveBoxes(p.stacks, m.start, m.end, m.count)
	}
	return tops(p.stacks)
}

func main() {
	fmt.Printf("part 1: %s\n", part1(input))
	fmt.Printf("part 2: %s\n", part2(input))
}
