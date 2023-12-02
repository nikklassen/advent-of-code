package main

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"
	"unicode"

	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
	"golang.org/x/exp/slices"
)

//go:embed input.txt
var input string

type segment interface {
	isList() bool
	toList() []segment
	value() int

	String() string
}

type listSegment struct {
	values []segment
}

func (*listSegment) isList() bool { return true }

func (s *listSegment) toList() []segment { return s.values }

func (s *listSegment) value() int {
	panic("cannot call value on a list")
}

func (s *listSegment) String() string {
	return "[" + strings.Join(aocslices.Map(s.values, func(s segment) string {
		return s.String()
	}), ",") + "]"
}

type intSegment struct {
	i int
}

func (*intSegment) isList() bool { return false }

func (s *intSegment) toList() []segment { return []segment{s} }

func (s *intSegment) value() int {
	return s.i
}

func (s *intSegment) String() string {
	return strconv.Itoa(s.i)
}

func compareInts(x, y int) int {
	if x == y {
		return 0
	}
	if x < y {
		return -1
	}
	return 1
}

func compare(p0, p1 segment) int {
	isList0 := p0.isList()
	isList1 := p1.isList()
	if !isList0 && !isList1 {
		return compareInts(p0.value(), p1.value())
	}
	l0 := p0.toList()
	l1 := p1.toList()
	for i := 0; i < len(l0) && i < len(l1); i++ {
		s0 := l0[i]
		s1 := l1[i]
		res := compare(s0, s1)
		if res != 0 {
			return res
		}
	}
	return compareInts(len(l0), len(l1))
}

func parsePacket(input string) *listSegment {
	root := &listSegment{}
	stack := []*listSegment{root}
	var currInt *int
	for _, c := range input[1:] {
		switch {
		case c == '[':
			curr := stack[len(stack)-1]
			newList := &listSegment{}
			curr.values = append(curr.values, newList)
			stack = append(stack, newList)
			continue
		case c == ']':
			if currInt != nil {
				curr := stack[len(stack)-1]
				curr.values = append(curr.values, &intSegment{*currInt})
				currInt = nil
			}
			stack = stack[:len(stack)-1]
		case unicode.IsDigit(c):
			if currInt == nil {
				tmp := 0
				currInt = &tmp
			}
			*currInt = *currInt*10 + (int(c) - '0')
		case c == ',':
			if currInt != nil {
				curr := stack[len(stack)-1]
				curr.values = append(curr.values, &intSegment{*currInt})
				currInt = nil
			}
		default:
			panic(fmt.Sprintf("invalid char: %v", c))
		}
	}
	if len(stack) != 0 {
		panic(fmt.Sprintf("stack has %d elements", len(stack)))
	}
	return root
}

func part1(input string) int {
	lines := aocstrings.Lines(input)
	c := 0
	for i := 0; i < len(lines); i += 3 {
		p0 := parsePacket(lines[i])
		p1 := parsePacket(lines[i+1])
		res := compare(p0, p1)
		if res != 1 {
			c += i/3 + 1
		}
	}
	return c
}

func part2(input string) int {
	lines := aocstrings.Lines(input)
	decode1 := &listSegment{values: []segment{&listSegment{values: []segment{&intSegment{i: 2}}}}}
	decode2 := &listSegment{values: []segment{&listSegment{values: []segment{&intSegment{i: 6}}}}}
	packets := []segment{decode1, decode2}
	for _, line := range lines {
		if line == "" {
			continue
		}
		p := parsePacket(line)
		packets = append(packets, p)
	}
	slices.SortFunc(packets, func(p1, p2 segment) bool {
		return compare(p1, p2) == -1
	})
	key := 1
	for i, p := range packets {
		if p == decode1 || p == decode2 {
			key *= i + 1
		}
	}
	return key
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
