package main

import (
	_ "embed"
	"fmt"
	"strings"

	"github.com/nikklassen/advent-of-code/shared/utils/aocmaps"
	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

var (
	//go:embed test_input.txt
	input string
)

type workflow struct {
	label string
	rules []rule
}

type rule struct {
	category rune
	op       rune
	value    int
	dest     string
}

func parseRule(line string) rule {
	line, dest, _ := strings.Cut(line, ":")
	category := line[0]
	op := line[1]
	return rule{
		category: rune(category),
		op:       rune(op),
		value:    aocstrings.MustAtoi(line[2:]),
		dest:     dest,
	}
}

func parseWorkflow(line string) workflow {
	label, rulesStr, _ := strings.Cut(line, "{")
	rulesStr = rulesStr[:len(rulesStr)-1]
	rules := aocslices.Map(strings.Split(rulesStr, ","), parseRule)
	return workflow{label, rules}
}

func part1(input string) int {
	paras := aocstrings.Paragraphs(input)
	workflows := aocslices.Map(aocstrings.Lines(paras[0]), parseWorkflow)
	workflowMap := aocmaps.FromSliceFunc(workflows, func(w workflow) string {
		return w.label
	})
	for _, part := range paras[1] {

	}
	return 0
}

// func part2(input string) int {
// 	return 0
// }

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	// fmt.Printf("part 2: %d\n", part2(input))
}
