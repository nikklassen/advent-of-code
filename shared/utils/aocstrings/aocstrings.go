package aocstrings

import (
	"bufio"
	"slices"
	"strconv"
	"strings"

	"github.com/nikklassen/advent-of-code/shared/utils"
)

func Lines(input string) []string {
	s := bufio.NewScanner(strings.NewReader(input))
	var ret []string
	for s.Scan() {
		ret = append(ret, s.Text())
	}
	return ret
}

func TryTrimPrefix(s string, prefix string) (string, bool) {
	s2 := strings.TrimPrefix(s, prefix)
	return s2, s2 != s
}

func Reverse(s string) string {
	b := []byte(s)
	slices.Reverse(b)
	return string(b)
}

func MustAtoi(s string) int {
	return utils.Must(strconv.Atoi(s))
}

func RuneGrid(s string) [][]rune {
	var grid [][]rune
	for _, row := range Lines(s) {
		grid = append(grid, []rune(row))
	}
	return grid
}
