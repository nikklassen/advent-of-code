package main

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"

	"github.com/nikklassen/advent-of-code/shared/utils"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

//go:embed input.txt
var input string

func paddedCommands(input string) []string {
	var commands []string
	for _, line := range aocstrings.Lines(input) {
		commands = append(commands, "noop")
		if line != "noop" {
			commands = append(commands, line)
		}
	}
	return commands
}

func part1(input string) int {
	commands := paddedCommands(input)
	x := 1
	sigStrength := 0
	for c := 1; c < 221; c++ {
		line := "noop"
		if c < len(commands) {
			line = commands[c-1]
		}
		if (c-20)%40 == 0 {
			sigStrength += x * c
		}
		if line != "noop" {
			_, vString, _ := strings.Cut(line, " ")
			x += utils.Must(strconv.Atoi(vString))
		}
	}
	return sigStrength
}

func part2(input string) string {
	commands := paddedCommands(input)
	sb := &strings.Builder{}
	x := 1
	for c := 0; c < 240; c++ {
		line := "noop"
		if c < len(commands) {
			line = commands[c]
		}
		pixel := c % 40
		if c > 0 && pixel == 0 {
			sb.WriteByte('\n')
		}
		if pixel >= x-1 && pixel <= x+1 {
			sb.WriteByte('#')
		} else {
			sb.WriteByte('.')
		}
		if line != "noop" {
			_, vString, _ := strings.Cut(line, " ")
			x += utils.Must(strconv.Atoi(vString))
		}
	}
	return sb.String()
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2:\n%s\n", part2(input))
}
