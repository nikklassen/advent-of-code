package aocslices

import "golang.org/x/exp/constraints"

func Repeat[E any](e E, n int) []E {
	s := make([]E, n)
	for i := 0; i < n; i++ {
		s[i] = e
	}
	return s
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

func Product[S ~[]E, E constraints.Integer | constraints.Float](s S) E {
	product := E(1)
	for _, e := range s {
		product *= e
	}
	return product
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

func Map[S ~[]E1, E1, E2 any](s S, f func(s E1) E2) []E2 {
	ret := make([]E2, 0, len(s))
	for _, e := range s {
		ret = append(ret, f(e))
	}
	return ret
}

func FlatMap[S ~[]E1, E1, E2 any](s S, f func(s E1) []E2) []E2 {
	ret := make([]E2, 0, len(s))
	for _, e := range s {
		ret = append(ret, f(e)...)
	}
	return ret
}

func TakeWhile[S ~[]E, E any](s S, f func(s E) bool) S {
	var ret S
	for _, e := range s {
		if !f(e) {
			break
		}
		ret = append(ret, e)
	}
	return ret
}

func Fold[S ~[]E, E, A any](s S, init A, f func(s E, a A) A) A {
	curr := init
	for _, e := range s {
		curr = f(e, curr)
	}
	return curr
}

func FoldR[S ~[]E, E, A any](s S, init A, f func(s E, a A) A) A {
	curr := init
	for i := len(s) - 1; i >= 0; i-- {
		curr = f(s[i], curr)
	}
	return curr
}

func ToSet[S ~[]E, E comparable](s S) map[E]bool {
	ret := map[E]bool{}
	for _, e := range s {
		ret[e] = true
	}
	return ret
}
