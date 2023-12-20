package main

import (
	_ "embed"
	"fmt"
	"strings"

	"github.com/nikklassen/advent-of-code/shared/utils/aocmaps"
	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
	"golang.org/x/exp/maps"
)

var (
	//go:embed input.txt
	input string
)

type workflow struct {
	label string
	rules []rule
}

type rule struct {
	category string
	op       string
	value    int
	dest     string
}

func (r rule) test(p part) bool {
	switch r.op {
	case "":
		return true
	case "<":
		return p[r.category] < r.value
	case ">":
		return p[r.category] > r.value
	default:
		panic("invalid op")
	}
}

func parseRule(line string) rule {
	line, dest, ok := strings.Cut(line, ":")
	if !ok {
		return rule{dest: line}
	}
	idx := strings.IndexAny(line, "><")
	category := line[:idx]
	op := line[idx : idx+1]
	return rule{
		category: category,
		op:       op,
		value:    aocstrings.MustAtoi(line[idx+1:]),
		dest:     dest,
	}
}

func parseWorkflow(line string) workflow {
	label, rulesStr, _ := strings.Cut(line, "{")
	rulesStr = rulesStr[:len(rulesStr)-1]
	rules := aocslices.Map(strings.Split(rulesStr, ","), parseRule)
	return workflow{label, rules}
}

type part map[string]int

func runWorkflow(p part, wm map[string]workflow) bool {
	curr := wm["in"]
	for {
		var next string
		for _, r := range curr.rules {
			if ok := r.test(p); ok {
				if r.dest == "R" {
					return false
				} else if r.dest == "A" {
					return true
				}
				next = r.dest
				break
			}
		}
		if next == "" {
			panic("not found")
		}
		curr = wm[next]
	}
}

func part1(input string) int {
	paras := aocstrings.Paragraphs(input)
	workflows := aocslices.Map(aocstrings.Lines(paras[0]), parseWorkflow)
	workflowMap := aocmaps.FromSliceFunc(workflows, func(w workflow) string {
		return w.label
	})
	var parts []part
	for _, line := range aocstrings.Lines(paras[1]) {
		attrs := strings.Split(line[1:len(line)-1], ",")
		p := part{}
		for _, attr := range attrs {
			p[attr[:1]] = aocstrings.MustAtoi(attr[2:])
		}
		parts = append(parts, p)
	}
	var tot int
	for _, p := range parts {
		if runWorkflow(p, workflowMap) {
			tot += aocslices.Sum(maps.Values(p))
		}
	}
	return tot
}

// func part2(input string) int {
// 	return 0
// }

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	// fmt.Printf("part 2: %d\n", part2(input))
}
