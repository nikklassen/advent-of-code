package main

import (
	_ "embed"
	"fmt"
	"slices"
	"strings"

	"github.com/nikklassen/advent-of-code/shared/utils"
	"github.com/nikklassen/advent-of-code/shared/utils/aocmaps"
	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
	"golang.org/x/exp/maps"
)

const maxValue = 4001

var (
	//go:embed test_input.txt
	input string
)

type workflow struct {
	label string
	rules []rule
}

type rule struct {
	category string
	op       string
	values   utils.Range
	dest     string
}

func (r rule) test(p part) bool {
	switch r.op {
	case "":
		return true
	case "<", ">":
		return r.values.Contains(p[r.category])
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
	value := aocstrings.MustAtoi(line[idx+1:])
	var values utils.Range
	if op == "<" {
		values = utils.Range{Start: 0, End: value}
	} else {
		values = utils.Range{Start: value + 1, End: maxValue}
	}
	return rule{
		category: category,
		op:       op,
		values:   values,
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

type step struct {
	r rule
	i int
}

func flip(r rule) rule {
	switch r.op {
	case "<":

		return rule{r.category, ">", utils.Range{Start: r.values.End - 1, End: maxValue}, r.dest}
	case ">":
		return rule{r.category, "<", utils.Range{Start: 0, End: r.values.Start + 1}, r.dest}
	}
	panic("agh!")
}

func merge(a, b rule) (rule, bool) {
	values := a.values.Intersect(b.values)
	if values == (utils.Range{}) {
		return rule{}, false
	}
	return rule{a.category, "", values, ""}, true
}

func mergeAll(rules []rule) map[string]utils.RangeSet {
	existing := map[string]utils.RangeSet{}
	for _, r := range rules {
		if _, ok := existing[r.category]; !ok {
			existing[r.category] = utils.RangeSet{r.values}
			continue
		}
		merged := existing[r.category].Intersect(utils.RangeSet{r.values})
		existing[r.category] = merged
	}
	return existing
}

func unionAll(all []map[string]utils.RangeSet) map[string]utils.RangeSet {
	ret := map[string]utils.RangeSet{}
	for _, m := range all {
		for _, k := range []string{"x", "m", "a", "s"} {
			ret[k] = ret[k].Union(m[k])
		}
	}
	return ret
}

func findAllAccepted(wm map[string]workflow, curr string, path []rule) []map[string]utils.RangeSet {
	rules := wm[curr].rules
	var ret []map[string]utils.RangeSet
	for _, r := range rules {
		if r.dest == "A" {
			// fmt.Println(r)
			all := slices.Clone(path)
			if r.category != "" {
				all = append(all, r)
			}
			merged := mergeAll(all)
			// fmt.Println(merged)
			ret = append(ret, merged)
			continue
		} else if r.dest == "R" {
			continue
		}
		var newRet []map[string]utils.RangeSet
		if r.category == "" {
			newRet = findAllAccepted(wm, r.dest, append(slices.Clone(path), aocslices.Map(rules[:len(rules)-1], flip)...))
		} else {
			newRet = findAllAccepted(wm, r.dest, append(slices.Clone(path), r))
		}
		ret = append(ret, newRet...)
	}
	return ret
}

func intersect(m1, m2 map[string]utils.RangeSet) map[string]utils.RangeSet {
	ret := map[string]utils.RangeSet{}
	for _, k := range []string{"x", "m", "a", "s"} {
		v1, ok := m1[k]
		if !ok {
			v1 = utils.RangeSet{{End: maxValue}}
		}
		v2, ok := m2[k]
		if !ok {
			v2 = utils.RangeSet{{End: maxValue}}
		}
		int := v1.Intersect(v2)
		if len(int) == 0 {
			return nil
		}
		ret[k] = int
	}
	return ret
}

func product(m map[string]utils.RangeSet) int {
	tot := 1
	for _, v := range m {
		tot *= v.Len()
	}
	return tot
}

func part2(input string) int {
	paras := aocstrings.Paragraphs(input)
	workflows := aocslices.Map(aocstrings.Lines(paras[0]), parseWorkflow)
	workflowMap := aocmaps.FromSliceFunc(workflows, func(w workflow) string {
		return w.label
	})
	all := findAllAccepted(workflowMap, "in", nil)
	for _, m := range all {
		fmt.Println(m)
	}
	fmt.Println()
	tot := 0
	for i, m1 := range all {
		tot += product(m1)
		for j := 0; j < i; j++ {
			tot -= product(intersect(m1, all[j]))
		}
	}
	return tot
}

func main() {
	// fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
