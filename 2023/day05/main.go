package main

import (
	_ "embed"
	"fmt"
	"slices"
	"strings"

	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

var (
	//go:embed input.txt
	input string
)

type span struct {
	start, length int
}

func (s span) end() int { return s.start + s.length }

type rangeMap struct {
	in, out []span
}

type mappingDef struct {
	in, out, length int
}

func newRangeMap(ranges [][]int) *rangeMap {
	slices.SortFunc(ranges, func(a, b []int) int {
		return a[1] - b[1]
	})
	m := &rangeMap{}
	for _, r := range ranges {
		m.in = append(m.in, span{r[1], r[2]})
		m.out = append(m.out, span{r[0], r[2]})
	}
	return m
}

func mapSpanByRange(in, mapping span) (mapped span, unmapped []span) {
	if in.end() < mapping.start || in.start >= mapping.end() {
		return span{}, []span{in}
	}
	mappedStart := min(max(mapping.start, in.start), mapping.end()-1)
	mappedEnd := max(min(mapping.end(), in.end()), in.start+1)
	if mappedStart > in.start {
		unmapped = append(unmapped, span{in.start, mappedStart - in.start})
	}
	if mappedEnd < in.end() {
		unmapped = append(unmapped, span{mappedEnd, in.end() - mappedEnd})
	}
	if mappedEnd == mappedStart {
		return span{}, unmapped
	}
	return span{mappedStart, mappedEnd - mappedStart}, unmapped
}

func mapSpanByRangeMap(rm *rangeMap, in span) []span {
	spansToMap := []span{in}
	var allMapped []span
	for i, mapping := range rm.in {
		var newPending []span
		for _, spanToMap := range spansToMap {
			mapped, unmapped := mapSpanByRange(spanToMap, mapping)
			if mapped.length > 0 {
				allMapped = append(allMapped, span{rm.out[i].start + mapped.start - mapping.start, mapped.length})
			}
			newPending = append(newPending, unmapped...)
		}
		spansToMap = newPending
		if len(spansToMap) == 0 {
			break
		}
	}
	return append(allMapped, spansToMap...)
}

func parseInput(input string) ([]int, []*rangeMap) {
	lines := aocstrings.Lines(input)
	var almanacMaps []*rangeMap
	i := 2
	for i < len(lines) {
		i++
		var ranges [][]int
		for i < len(lines) && lines[i] != "" {
			ranges = append(ranges, aocstrings.SpaceSeparatedInts(lines[i]))
			i++
		}
		almanacMaps = append(almanacMaps, newRangeMap(ranges))
		i++
	}
	return aocstrings.SpaceSeparatedInts(strings.TrimPrefix(lines[0], "seeds: ")), almanacMaps
}

func applyAlmanacToSpansAndFindMin(in []span, almanac []*rangeMap) int {
	current := in
	for _, m := range almanac {
		var newCurrent []span
		for _, c := range current {
			newCurrent = append(newCurrent, mapSpanByRangeMap(m, c)...)
		}
		current = newCurrent
	}
	return slices.MinFunc(current, func(a, b span) int {
		return a.start - b.start
	}).start
}

func findMinLocationWithSingleSeeds(input string) int {
	seeds, almanac := parseInput(input)
	var seedSpans []span
	for _, seed := range seeds {
		seedSpans = append(seedSpans, span{seed, 1})
	}
	return applyAlmanacToSpansAndFindMin(seedSpans, almanac)
}

func part1(input string) int {
	return findMinLocationWithSingleSeeds(input)
}

func findMinLocationWithSeedSpans(input string) int {
	seeds, almanac := parseInput(input)
	var seedSpans []span
	for i := 0; i < len(seeds); i += 2 {
		seedSpans = append(seedSpans, span{seeds[i], seeds[i+1]})
	}
	return applyAlmanacToSpansAndFindMin(seedSpans, almanac)
}

func part2(input string) int {
	return findMinLocationWithSeedSpans(input)
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
