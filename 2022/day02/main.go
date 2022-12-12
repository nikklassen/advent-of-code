package main

import (
	_ "embed"
	"fmt"
	"strings"

	"github.com/nikklassen/advent-of-code/2022/utils/aocslices"
	"github.com/nikklassen/advent-of-code/2022/utils/aocstrings"
)

//go:embed input.txt
var input string

type RPS int

const (
	Rock RPS = iota + 1
	Paper
	Scissors
)

type game struct {
	opponent RPS
	col2     string
}

func parseGame(line string) game {
	parts := strings.Split(line, " ")
	var opponent RPS
	switch parts[0] {
	case "A":
		opponent = Rock
	case "B":
		opponent = Paper
	case "C":
		opponent = Scissors
	}
	return game{
		opponent: opponent,
		col2:     parts[1],
	}
}

func score(opponent, player RPS) int {
	var shape, outcome int
	switch player {
	case Rock:
		shape = 1
		switch opponent {
		case Rock:
			outcome = 3
		case Scissors:
			outcome = 6
		}
	case Paper:
		shape = 2
		switch opponent {
		case Rock:
			outcome = 6
		case Paper:
			outcome = 3
		}
	case Scissors:
		shape = 3
		switch opponent {
		case Paper:
			outcome = 6
		case Scissors:
			outcome = 3
		}
	}
	return shape + outcome
}

func part1(input string) int {
	return aocslices.Sum(aocslices.Map(aocstrings.Lines(input), func(line string) int {
		g := parseGame(line)
		var player RPS
		switch g.col2 {
		case "X":
			player = Rock
		case "Y":
			player = Paper
		case "Z":
			player = Scissors
		}
		return score(g.opponent, player)
	}))
}

func part2(input string) int {
	return aocslices.Sum(aocslices.Map(aocstrings.Lines(input), func(line string) int {
		g := parseGame(line)
		var player RPS
		switch g.col2 {
		// Lose
		case "X":
			switch g.opponent {
			case Rock:
				player = Scissors
			case Paper:
				player = Rock
			case Scissors:
				player = Paper
			}
		// Draw
		case "Y":
			switch g.opponent {
			case Rock:
				player = Rock
			case Paper:
				player = Paper
			case Scissors:
				player = Scissors
			}
		// Win
		case "Z":
			switch g.opponent {
			case Rock:
				player = Paper
			case Paper:
				player = Scissors
			case Scissors:
				player = Rock
			}
		}
		return score(g.opponent, player)
	}))
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
