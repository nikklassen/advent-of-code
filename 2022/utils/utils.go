package utils

import (
	"bufio"
	"strings"

	"golang.org/x/exp/constraints"
)

func Map[S ~[]E1, E1, E2 any](s S, f func(s E1) E2) []E2 {
	ret := make([]E2, 0, len(s))
	for _, e := range s {
		ret = append(ret, f(e))
	}
	return ret
}

func Lines(input string) []string {
	s := bufio.NewScanner(strings.NewReader(input))
	var ret []string
	for s.Scan() {
		ret = append(ret, s.Text())
	}
	return ret
}

func MapLines[E any](input string, f func(s string) E) []E {
	var ret []E
	ForEachLines(input, func(s string) {
		ret = append(ret, f(s))
	})
	return ret
}

func ForEachLines(input string, f func(s string)) {
	s := bufio.NewScanner(strings.NewReader(input))
	for s.Scan() {
		line := s.Text()
		f(line)
	}
}

func Chunks[S ~[]E, E any](s S, n int) []S {
	var ret []S
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

func Must[T any](t T, e error) T {
	if e != nil {
		panic(e)
	}
	return t
}

func Filter[S ~[]E, E any](s []E, f func(E) bool) []E {
	var ret S
	for _, t := range s {
		if f(t) {
			ret = append(ret, t)
		}
	}
	return ret
}

func FilterInplace[S ~[]E, E any](s S, f func(E) bool) []E {
	var i int
	for j, t := range s {
		if f(t) {
			if i != j {
				s[i] = s[j]
			}
			i++
		}
	}
	return s
}

func CountFunc[S ~[]E, E any](s S, f func(e E) bool) int {
	var c int
	for _, e := range s {
		if f(e) {
			c++
		}
	}
	return c
}

func Sum[S ~[]E, E constraints.Integer | constraints.Float](s S) E {
	var tot E
	for _, e := range s {
		tot += e
	}
	return tot
}

func TryTrimPrefix(s string, prefix string) (string, bool) {
	s2 := strings.TrimPrefix(s, prefix)
	return s2, s2 != s
}
