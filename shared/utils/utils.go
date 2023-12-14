package utils

type Tuple[T1, T2 any] struct {
	Item1 T1
	Item2 T2
}

type Tuple3[T1, T2, T3 any] struct {
	Item1 T1
	Item2 T2
	Item3 T3
}

func Must[T any](t T, e error) T {
	if e != nil {
		panic(e)
	}
	return t
}
