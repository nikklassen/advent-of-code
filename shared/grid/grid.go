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

func I(x, y int) Index {
	return Index{X: x, Y: y}
}

type Grid[T any] [][]T

func (g Grid[T]) Lookup(i Index) (T, bool) {
	if i.Y >= 0 && i.Y < len(g) && i.X >= 0 && i.X < len(g[i.Y]) {
		return g[i.Y][i.X], true
	}
	var zero T
	return zero, false
}

// Get returns the value at i if i is within the bounds of the grid, otherwise the zero value for T.
// To test if i is valid use Lookup instead.
func (g Grid[T]) Get(i Index) T {
	v, _ := g.Lookup(i)
	return v
}

func (g Grid[T]) Set(i Index, t T) {
	g[i.Y][i.X] = t
}

func NewGridSize[T any](cols, rows int) Grid[T] {
	var g Grid[T]
	for y := 0; y < rows; y++ {
		g = append(g, make([]T, cols))
	}
	return g
}

func Adjacent(i Index) []Index {
	var ret []Index
	for x := i.X - 1; x < i.X+2; x++ {
		for y := i.Y - 1; y < i.Y+2; y++ {
			ret = append(ret, Index{X: x, Y: y})
		}
	}
	return ret
}
