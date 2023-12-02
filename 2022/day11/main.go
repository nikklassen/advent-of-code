package main

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"

	"github.com/nikklassen/advent-of-code/shared/utils"
	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

//go:embed input.txt
var input string

type monkey struct {
	num   int
	items []int
	op    monkeyOp
	test  monkeyTest
}

func (m *monkey) String() string {
	s := fmt.Sprintf("Monkey %d: ", m.num)
	s += strings.Join(aocslices.Map(m.items, strconv.Itoa), ", ")
	return s
}

type monkeyOp struct {
	op    string
	value string
}

type monkeyTest struct {
	divisor int
	ifTrue  int
	ifFalse int
}

func parseMonkey(lines []string) *monkey {
	m := &monkey{}
	utils.Must(fmt.Sscanf(lines[0], "Monkey %d:", &m.num))
	startingLine := strings.TrimPrefix(lines[1], "  Starting items: ")
	for _, item := range strings.Split(startingLine, ", ") {
		m.items = append(m.items, utils.Must(strconv.Atoi(item)))
	}
	utils.Must(fmt.Sscanf(lines[2], "  Operation: new = old %s %s", &m.op.op, &m.op.value))
	utils.Must(fmt.Sscanf(lines[3], "  Test: divisible by %d", &m.test.divisor))
	utils.Must(fmt.Sscanf(lines[4], "    If true: throw to monkey %d", &m.test.ifTrue))
	utils.Must(fmt.Sscanf(lines[5], "    If false: throw to monkey %d", &m.test.ifFalse))
	return m
}

func parseInput(input string) []*monkey {
	lines := aocstrings.Lines(input)
	var monkeys []*monkey
	for i := 0; i < len(lines); i += 7 {
		monkeys = append(monkeys, parseMonkey(lines[i:i+6]))
	}
	return monkeys
}

func evaluateOp(i int, op monkeyOp) int {
	var val int
	if op.value == "old" {
		val = i
	} else {
		val = utils.Must(strconv.Atoi(op.value))
	}
	switch op.op {
	case "*":
		return i * val
	case "/":
		return i / val
	case "+":
		return i + val
	case "-":
		return i - val
	default:
		panic("unknown op: " + op.op)
	}
}

func evaluateRounds(monkeys []*monkey, rounds int, relief bool) []int {
	lcm := 1
	for _, m := range monkeys {
		lcm *= m.test.divisor
	}
	counters := make([]int, len(monkeys))
	for r := 0; r < rounds; r++ {
		for i, monkey := range monkeys {
			for _, item := range monkey.items {
				counters[i]++
				newItem := evaluateOp(item, monkey.op)
				if relief {
					newItem /= 3
				}
				newItem %= lcm
				var throwTo int
				if newItem%monkey.test.divisor == 0 {
					throwTo = monkey.test.ifTrue
				} else {
					throwTo = monkey.test.ifFalse
				}
				monkeys[throwTo].items = append(monkeys[throwTo].items, newItem)
			}
			monkey.items = nil
		}
	}
	return counters
}

func monkeyBusiness(counters []int) int {
	max1, max2 := 0, 0
	for _, c := range counters {
		if c > max1 {
			max2 = max1
			max1 = c
		} else if c > max2 {
			max2 = c
		}
	}
	return max1 * max2
}

func part1(input string) int {
	monkeys := parseInput(input)
	counters := evaluateRounds(monkeys, 20, true)
	return monkeyBusiness(counters)
}

func part2(input string) int {
	monkeys := parseInput(input)
	counters := evaluateRounds(monkeys, 10000, false)
	return monkeyBusiness(counters)
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
