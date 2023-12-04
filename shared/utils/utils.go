package utils

import "strconv"

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

func MustAtoi(s string) int {
	return Must(strconv.Atoi(s))
}
