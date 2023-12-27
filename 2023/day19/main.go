package main

import (
	"cmp"
	_ "embed"
	"fmt"
	"slices"
	"strings"

	"github.com/nikklassen/advent-of-code/shared/utils"
	"github.com/nikklassen/advent-of-code/shared/utils/aocmaps"
	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
	"golang.org/x/exp/maps"
	"golang.org/x/text/language"
	"golang.org/x/text/message"
)

const maxValue = 4001

var (
	//go:embed test_input.txt
	input    string
	allAttrs = []string{"x", "m", "a", "s"}
)

type workflow struct {
	label string
	rules []rule
}

type rule struct {
	category string
	values   utils.Range
	dest     string
}

func (r rule) test(p part) bool {
	if r.category == "" {
		return true
	}
	return r.values.Contains(p[r.category])
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
		values = utils.Range{Start: 1, End: value}
	} else {
		values = utils.Range{Start: value + 1, End: maxValue}
	}
	return rule{
		category: category,
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

type partCondition map[string]utils.RangeSet

func (pc partCondition) attrString(a string) string {
	v, ok := pc[a]
	if !ok {
		return ""
	}
	return fmt.Sprintf("%v", v)
}

func (pc partCondition) String() string {
	return fmt.Sprintf("x: %s, m: %s, a: %s, s: %s", pc.attrString("x"), pc.attrString("m"), pc.attrString("a"), pc.attrString("s"))
}

func flip(r rule) rule {
	if r.values.Start == 0 {
		return rule{r.category, utils.Range{Start: r.values.End, End: maxValue}, r.dest}
	}
	return rule{r.category, utils.Range{Start: 1, End: r.values.Start}, r.dest}
}

func merge(a, b rule) (rule, bool) {
	values := a.values.Intersect(b.values)
	if values == (utils.Range{}) {
		return rule{}, false
	}
	return rule{a.category, values, ""}, true
}

func mergeAll(rules []rule) partCondition {
	existing := partCondition{}
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

func unionAll(all []partCondition) partCondition {
	ret := partCondition{}
	for _, m := range all {
		for _, k := range allAttrs {
			_, ok := m[k]
			_, existing := ret[k]
			if !ok && !existing {
				continue
			}
			ret[k] = ret[k].Union(m[k])
		}
	}
	return ret
}

func findAllAccepted(wm map[string]workflow, curr string, path []rule) []partCondition {
	rules := wm[curr].rules
	var ret []partCondition
	for _, r := range rules {
		newPath := slices.Clone(path)
		if r.category == "" {
			newPath = append(newPath, aocslices.Map(rules[:len(rules)-1], flip)...)
		} else {
			newPath = append(newPath, r)
		}
		if r.dest == "A" {
			slices.SortFunc(newPath, func(a, b rule) int {
				return cmp.Compare(slices.Index(allAttrs, a.category), slices.Index(allAttrs, b.category))
			})
			// fmt.Println("accepted path:", newPath)
			merged := mergeAll(newPath)
			// fmt.Println("merged:", merged)
			ret = append(ret, merged)
			continue
		} else if r.dest == "R" {
			continue
		} else {
			ret = append(ret, findAllAccepted(wm, r.dest, newPath)...)
		}
	}
	return ret
}

func intersect(m1, m2 partCondition) partCondition {
	ret := partCondition{}
	for _, k := range allAttrs {
		v1, ok := m1[k]
		if !ok {
			v1 = utils.RangeSet{{Start: 1, End: maxValue}}
		}
		v2, ok := m2[k]
		if !ok {
			v2 = utils.RangeSet{{Start: 1, End: maxValue}}
		}
		int := v1.Intersect(v2)
		if len(int) == 0 {
			return nil
		}
		ret[k] = int
	}
	return ret
}

func product(m partCondition) int {
	if len(m) == 0 {
		return 0
	}
	tot := 1
	for _, a := range allAttrs {
		v, ok := m[a]
		if !ok {
			tot *= maxValue - 1
		} else {
			tot *= v.Len()
		}
	}
	return tot
}

type tempDiffPart struct {
	bPart bool
	pc    partCondition
}

func difference(a, b partCondition) []partCondition {
	ret := []tempDiffPart{{
		bPart: true,
		pc:    partCondition{},
	}}
	for _, attr := range allAttrs {
		v := a[attr]
		d := v.Difference(b[attr])
		var nextRet []tempDiffPart
		for _, p := range ret {
			if len(d) > 0 {
				c := maps.Clone(p.pc)
				c[attr] = d
				nextRet = append(nextRet, tempDiffPart{
					bPart: false,
					pc:    c,
				})
			}
			if !slices.Equal(v, d) && len(b[attr]) > 0 {
				c := maps.Clone(p.pc)
				c[attr] = b[attr]
				nextRet = append(nextRet, tempDiffPart{
					bPart: p.bPart,
					pc:    c,
				})
			}
		}
		ret = nextRet
	}
	var final []partCondition
	for _, p := range ret {
		if !p.bPart {
			final = append(final, p.pc)
		}
	}
	return final
}

func printMap(m partCondition) {
	hasValue := false
	for _, k := range allAttrs {
		if v, ok := m[k]; ok {
			hasValue = true
			fmt.Print(aocstrings.PadRight(fmt.Sprintf("%v: %v", k, v), 30, ' '))
		} else {
			fmt.Print(strings.Repeat(" ", 30))
		}
	}
	if !hasValue {
		fmt.Print("<empty>")
	}
	fmt.Println()
}

// func part2(input string) int {
// 	paras := aocstrings.Paragraphs(input)
// 	workflows := aocslices.Map(aocstrings.Lines(paras[0]), parseWorkflow)
// 	workflowMap := aocmaps.FromSliceFunc(workflows, func(w workflow) string {
// 		return w.label
// 	})
// 	all := findAllAccepted(workflowMap, "in", nil)
// 	for _, m := range all {
// 		fmt.Println(m)
// 	}
// 	for _, a := range all {
// 		for _, attr := range allAttrs {
// 			if _, ok := a[attr]; !ok {
// 				a[attr] = utils.RangeSet{{End: maxValue}}
// 			}
// 		}
// 	}
// 	fmt.Println()
// 	var prev []partCondition
// 	for !slices.EqualFunc(prev, all, func(a, b partCondition) bool {
// 		for _, attr := range allAttrs {
// 			if !slices.Equal(a[attr], b[attr]) {
// 				return false
// 			}
// 		}
// 		return true
// 	}) {
// 		prev = all
// 		var i, j int
// 		var a, b partCondition
// 		var intersection partCondition
// 	findLoop:
// 		for i, a = range all {
// 			for j = i + 1; j < len(all); j++ {
// 				b = all[j]
// 				intersection = intersect(a, b)
// 				if len(intersection) > 0 {
// 					break findLoop
// 				}
// 			}
// 		}
// 		if len(intersection) == 0 {
// 			break
// 		}
// 		fmt.Println("found intersection of ")
// 		printMap(a)
// 		fmt.Println("and")
// 		printMap(b)
// 		fmt.Println("=")
// 		printMap(intersection)
// 		fmt.Println()
// 		nextAll := slices.Clone(all[:i])
// 		nextAll = append(nextAll, difference(a, intersection)...)
// 		nextAll = append(nextAll, all[i+1:j]...)
// 		nextAll = append(nextAll, difference(b, intersection)...)
// 		nextAll = append(nextAll, all[i+1:j]...)
// 		nextAll = append(nextAll, intersection)
// 		nextAll = append(nextAll, all[j+1:]...)
// 		all = nextAll
// 	}
// 	fmt.Println("All:")
// 	for _, a := range all {
// 		printMap(a)
// 	}
// 	tot := 0
// 	for _, a := range all {
// 		tot += product(a)
// 	}
// 	p := message.NewPrinter(language.English)
// 	want := 167409079868000
// 	p.Printf("want: % 20d\n", want)
// 	p.Printf("got:  % 20d\n", tot)
// 	return tot
// }

func ranges(attr string, all []partCondition) []utils.Range {
	pointSet := map[int]bool{}
	for _, m := range all {
		for _, r := range m[attr] {
			pointSet[r.Start] = true
			pointSet[r.End-1] = true
		}
	}
	points := maps.Keys(pointSet)
	slices.Sort(points)
	prev := points[0]
	var ret []utils.Range
	for _, p := range points[1:] {
		ret = append(ret, utils.Range{Start: prev, End: p + 1})
		prev = p + 1
	}
	// ret = append(ret, utils.Range{Start: prev, End: maxValue})
	return ret
}

func countAll(attrs []string, ms []partCondition) int {
	if len(attrs) == 0 {
		return 1
	}
	attr := attrs[0]
	fmt.Println("attr", attr)
	rs := ranges(attr, ms)
	fmt.Println(rs)
	tot := 0
	for _, r := range rs {
		var next []partCondition
		for _, m := range ms {
			if m[attr].Intersect(utils.RangeSet{r}).Len() > 0 {
				next = append(next, m)
			}
		}
		tot += r.Len() * len(next) * countAll(attrs[1:], next)
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
		for _, attr := range allAttrs {
			if r, ok := m[attr]; !ok || r.Len() == 0 {
				m[attr] = utils.RangeSet{{Start: 1, End: maxValue}}
			}
		}
	}
	for _, m := range all {
		fmt.Println(m)
	}
	fmt.Println()
	return countAll(allAttrs, all)
}

func main() {
	// fmt.Printf("part 1: %d\n", part1(input))
	p := message.NewPrinter(language.English)
	p.Printf("part 2: %d\n", part2(input))
	p.Printf("%d\n", 167409079868000)
}
