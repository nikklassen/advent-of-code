package aocstrings

import (
	"bufio"
	"slices"
	"strings"
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
