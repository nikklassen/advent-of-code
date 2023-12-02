package grid

type Index struct{ X, Y int }

func (i Index) Add(i2 Index) Index {
	return Index{
		X: i.X + i2.X,
		Y: i.Y + i2.Y,
	}
}

func (i Index) Sub(i2 Index) Index {
	return Index{
		X: i.X - i2.X,
		Y: i.Y - i2.Y,
	}
}

type Grid[T any] [][]T

func (g Grid[T]) Get(i Index) (T, bool) {
	if i.Y >= 0 && i.Y < len(g) && i.X >= 0 && i.X < len(g[i.Y]) {
		return g[i.Y][i.X], true
	}
	var zero T
	return zero, false
}

func (g Grid[T]) Set(i Index, t T) {
	g[i.Y][i.X] = t
}
