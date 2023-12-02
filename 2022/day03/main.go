package main

import (
	_ "embed"
	"fmt"
	"unicode"

	"github.com/nikklassen/advent-of-code/shared/utils"
	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

//go:embed input.txt
var input string

type Rucksack utils.Tuple[string, string]

func parseRucksack(line string) Rucksack {
	return Rucksack{
		Item1: line[:len(line)/2],
		Item2: line[len(line)/2:],
	}
}

func priority(r rune) int {
	if unicode.IsLower(r) {
		return int(byte(r)-'a') + 1
	}
	return int(byte(r)-'A') + 27
}

func part1(input string) int {
	return aocslices.Sum(
		aocslices.Map(
			aocslices.Map(aocstrings.Lines(input), parseRucksack),
			func(r Rucksack) int {
				items := map[rune]bool{}
				for _, i := range r.Item1 {
					items[i] = true
				}
				for _, i := range r.Item2 {
					if items[i] {
						return priority(i)
					}
				}
				return 0
			},
		))
}

func part2(input string) int {
	return aocslices.Sum(
		aocslices.Map(
			aocslices.Chunks(aocslices.Map(aocstrings.Lines(input), parseRucksack), 3),
			func(rs []Rucksack) int {
				counts := map[rune]int{}
				for _, r := range rs {
					items := map[rune]bool{}
					for _, i := range r.Item1 {
						items[i] = true
					}
					for _, i := range r.Item2 {
						items[i] = true
					}
					for i := range items {
						counts[i] += 1
					}
				}
				for i, c := range counts {
					if c == 3 {
						return priority(i)
					}
				}
				return 0
			},
		))
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
