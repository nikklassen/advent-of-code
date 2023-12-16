package main

import (
	_ "embed"
	"fmt"
	"slices"
	"strings"

	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

var (
	//go:embed input.txt
	input string
)

func hash(s string) int {
	return aocslices.Fold([]byte(s), 0, func(c byte, acc int) int {
		return ((acc + int(c)) * 17) % 256
	})
}

func part1(input string) int {
	input = strings.TrimSpace(input)
	return aocslices.Sum(aocslices.Map(strings.Split(input, ","), hash))
}

type lens struct {
	label string
	value int
}

func removeLabel(box []lens, label string) []lens {
	i := slices.IndexFunc(box, func(l lens) bool {
		return l.label == label
	})
	if i == -1 {
		return box
	}
	return append(box[:i], box[i+1:]...)
}

func insertLabel(box []lens, l lens) []lens {
	i := slices.IndexFunc(box, func(existing lens) bool {
		return existing.label == l.label
	})
	if i == -1 {
		return append(box, l)
	}
	box[i] = l
	return box
}

func part2(input string) int {
	insts := strings.Split(strings.TrimSpace(input), ",")
	boxes := make([][]lens, 256)
	for _, inst := range insts {
		var h int
		if label, ok := aocstrings.TryTrimSuffix(inst, "-"); ok {
			h = hash(label)
			boxes[h] = removeLabel(boxes[h], label)
		} else {
			label, value, _ := strings.Cut(inst, "=")
			h = hash(label)
			boxes[h] = insertLabel(boxes[h], lens{label, aocstrings.MustAtoi(value)})
		}
	}
	var ret int
	for i, box := range boxes {
		for j, l := range box {
			ret += (i + 1) * (j + 1) * l.value
		}
	}
	return ret
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
