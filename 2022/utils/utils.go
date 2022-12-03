package utils

import (
	"bufio"
	"strings"
)

func ParseInput[T any](input string, parser func(s string) T) []T {
	s := bufio.NewScanner(strings.NewReader(input))
	var ret []T
	for s.Scan() {
		line := s.Text()
		ret = append(ret, parser(line))
	}
	return ret
}

func Chunks[T any](s []T, n int) [][]T {
	var ret [][]T
	for i := 0; i < len(s); i += n {
		end := i + n
		if end > len(s) {
			end = len(s)
		}
		ret = append(ret, s[i:end])
	}
	return ret
}

type Tuple[T1, T2 any] struct {
	Item1 T1
	Item2 T2
}
