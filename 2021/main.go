package main

import (
	"fmt"
	"time"

	"github.com/nikklassen/advent-of-code/2021/day18"
)

func main() {
	start := time.Now()
	fmt.Printf("part1: %d\n", day18.Part1())
	fmt.Printf("elapsed: %v\n", time.Since(start))

	start = time.Now()
	fmt.Printf("part2: %d\n", day18.Part2())
	fmt.Printf("elapsed: %v\n", time.Since(start))
}
