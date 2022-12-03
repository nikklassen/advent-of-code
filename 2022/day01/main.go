package main

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"

	"golang.org/x/exp/slices"
)

//go:embed input.txt
var input string

func computeSumsAsc(input string) []int {
	var sums []int
	var sum int
	for _, line := range strings.Split(input, "\r\n") {
		if line == "" {
			sums = append(sums, sum)
			sum = 0
			continue
		}
		v, err := strconv.Atoi(line)
		if err != nil {
			panic(err)
		}
		sum += v
	}
	slices.Sort(sums)
	return sums
}

func part1(input string) int {
	sums := computeSumsAsc(input)
	return sums[len(sums)-1]
}

func part2(input string) int {
	sums := computeSumsAsc(input)

	var tot int
	for _, sum := range sums[len(sums)-3:] {
		tot += sum
	}

	return tot
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
