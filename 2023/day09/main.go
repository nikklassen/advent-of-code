package main

import (
	_ "embed"
	"fmt"

	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

var (
	//go:embed input.txt
	input string
)

func getNextNumber(seq []int, start bool) int {
	var boundaryNums []int
	for {
		if start {
			boundaryNums = append(boundaryNums, seq[0])
		} else {
			boundaryNums = append(boundaryNums, seq[len(seq)-1])
		}
		var nextSeq []int
		end := true
		for i := 1; i < len(seq); i++ {
			diff := seq[i] - seq[i-1]
			if diff != 0 {
				end = false
			}
			nextSeq = append(nextSeq, diff)
		}
		if end {
			break
		}
		seq = nextSeq
	}
	fn := func(x, y int) int { return x + y }
	if start {
		fn = func(x, acc int) int { return x - acc }
	}
	return aocslices.FoldR(boundaryNums, 0, fn)
}

func sumBoundaryNumbers(input string, start bool) int {
	seqs := aocslices.Map(aocstrings.Lines(input), aocstrings.SpaceSeparatedInts)
	return aocslices.Sum(aocslices.Map(seqs, func(seq []int) int {
		return getNextNumber(seq, start)
	}))
}

func part1(input string) int {
	return sumBoundaryNumbers(input, false)
}

func part2(input string) int {
	return sumBoundaryNumbers(input, true)
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
