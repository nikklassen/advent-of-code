package main

import (
	_ "embed"
	"fmt"
	"slices"
	"strings"

	"github.com/nikklassen/advent-of-code/shared/utils/aocmath"
	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
	"golang.org/x/exp/maps"
	"golang.org/x/text/language"
	"golang.org/x/text/message"
)

var (
	//go:embed input.txt
	input string

	debug = false
)

type moduleType int

const (
	broadcasterModule moduleType = iota + 1
	buttonModule
	flipFlopModule
	conjunctionModule
)

type signal struct {
	p            pulse
	source, dest string
}

type module interface {
	Receive(source string, p pulse) []signal
	Label() string
	Outputs() []string
}

type baseModule struct {
	label   string
	outputs []string
}

func (bm *baseModule) Label() string { return bm.label }

func (bm *baseModule) Outputs() []string { return bm.outputs }

func (bm *baseModule) publish(p pulse) []signal {
	return aocslices.Map(bm.outputs, func(d string) signal {
		return signal{p, bm.label, d}
	})
}

type pulse bool

type flipFlop struct {
	baseModule

	state        bool
	destinations []string
}

func (ff *flipFlop) Receive(_ string, p pulse) []signal {
	if p {
		return nil
	}
	ff.state = !ff.state
	return ff.publish(pulse(ff.state))
}

type conjunction struct {
	baseModule

	inputs map[string]pulse
}

func (c *conjunction) Receive(source string, p pulse) []signal {
	c.inputs[source] = p
	out := aocslices.Fold(maps.Values(c.inputs), true, func(s, a pulse) pulse {
		return s && a
	})
	return c.publish(!out)
}

type broadcaster struct {
	outputs []string
}

func parseModules(input string) (*broadcaster, map[string]module) {
	lines := aocstrings.Lines(input)
	var b *broadcaster
	ret := map[string]module{}
	for _, l := range lines {
		label, outputStr, _ := strings.Cut(l[1:], " -> ")
		bm := baseModule{
			label:   label,
			outputs: strings.Split(outputStr, ", "),
		}
		switch l[0] {
		case '&':
			ret[label] = &conjunction{
				baseModule: bm,
				inputs:     map[string]pulse{},
			}
		case '%':
			ret[label] = &flipFlop{
				baseModule: bm,
			}
		default:
			b = &broadcaster{outputs: bm.outputs}
		}
	}
	return b, ret
}

func buildModules(input string) (*broadcaster, map[string]module) {
	b, modules := parseModules(input)
	for _, m := range modules {
		for _, out := range m.Outputs() {
			if c, ok := modules[out].(*conjunction); ok {
				c.inputs[m.Label()] = false
			}
		}
	}
	return b, modules
}

func pushButton(b *broadcaster, modules map[string]module) []signal {
	var pendingSignals []signal
	for _, out := range b.outputs {
		if c, ok := modules[out].(*conjunction); ok {
			c.inputs["broadcaster"] = false
		}
		pendingSignals = append(pendingSignals, signal{source: "broadcaster", dest: out})
	}
	return pendingSignals
}

func part1(input string) int {
	b, modules := buildModules(input)
	pendingSignals := pushButton(b, modules)
	initial := slices.Clone(pendingSignals)
	low := 0
	high := 0
	for i := 0; i < 1000; i++ {
		pendingSignals = slices.Clone(initial)
		low += len(pendingSignals) + 1
		for len(pendingSignals) > 0 {
			next := pendingSignals[0]
			pendingSignals = pendingSignals[1:]
			destMod, ok := modules[next.dest]
			if !ok {
				continue
			}
			newSignals := destMod.Receive(next.source, next.p)
			for _, s := range newSignals {
				if s.p {
					high++
				} else {
					low++
				}
			}
			pendingSignals = append(pendingSignals, newSignals...)
		}
	}
	return high * low
}

func tmp(input string) int {
	b, modules := buildModules(input)
	pendingSignals := pushButton(b, modules)
	initial := slices.Clone(pendingSignals)
	i := 0
	// rx := 0
	fmt.Println("dk\tfg\tpq\tfm")
outer:
	for ; ; i++ {
		pendingSignals = slices.Clone(initial)
		for len(pendingSignals) > 0 {
			next := pendingSignals[0]
			pendingSignals = pendingSignals[1:]
			destMod, ok := modules[next.dest]
			if !ok {
				// rx++
				// continue
				if !next.p {
					break outer
				}
				continue
			}
			// if next.dest == "vr" && next.p {
			// fmt.Println(next.source, i)
			// }
			if !next.p {
				switch destMod.Label() {
				case "dk":
					fmt.Println(i)
				case "fg":
					fmt.Println("\t", i)
				case "pq":
					fmt.Println("\t\t", i)
				case "fm":
					fmt.Println("\t\t\t", i)
				}
			}
			newSignals := destMod.Receive(next.source, next.p)
			pendingSignals = append(pendingSignals, newSignals...)
		}
		// if rx == 1 {
		// 	break
		// }
		// rx = 0
	}
	return i
}

// func part2(_ string) int {
// 	// var k1, k2, k3, k4 uint = 3792, 3928, 4000, 4006
// 	var k1, k2, k3, k4 uint = 2, 3, 5, 7
// 	var i1, i2, i3, i4 uint = 1, 1, 1, 1
// 	t1, t2, t3, t4 := k1, k2, k3, k4

// 	for {
// 		x := min(t1, t2, t3, t4)
// 		switch x {
// 		case t1:
// 			t1 += k1 + 1
// 			i1++
// 		case t2:
// 			t2 += k2 + 1
// 			i2++
// 		case t3:
// 			t3 += k3 + 1
// 			i3++
// 		case t4:
// 			t4 += k4 + 1
// 			i4++
// 		}
// 		// fmt.Println(t1, t2, t3, t4)
// 		if t1 == t2 && t1 == t3 && t1 == t4 {
// 			return int(t1)
// 		}
// 		// time.Sleep(10 * time.Millisecond)
// 	}
// }

func part2(_ string) int {
	var k1, k2, k3, k4 uint = 3792, 3928, 4000, 4006
	return int(aocmath.LCMAll([]uint{k1 + 1, k2 + 1, k3 + 1, k4 + 1})) - 1
}

func main() {
	// fmt.Printf("part 1: %d\n", part1(input))
	// fmt.Printf("tmp: %d\n", tmp(input))
	fmt.Printf("part 2: %d\n", part2(input))
	p := message.NewPrinter(language.English)
	p.Printf("part 2: %d\n", part2(input))
}
