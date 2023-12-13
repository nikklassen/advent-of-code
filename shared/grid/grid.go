package grid

import "fmt"

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

// Cells of the grid in row-major order.
func (g Grid[T]) Cells() []T {
	ret := make([]T, 0, len(g)*len(g[0]))
	for _, row := range g {
		ret = append(ret, row...)
	}
	return ret
}

type IndexedCell[T any] struct {
	Idx  Index
	Cell T
}

// Cells of the grid in row-major order.
func (g Grid[T]) IndexedCells() []IndexedCell[T] {
	ret := make([]IndexedCell[T], 0, len(g)*len(g[0]))
	for y, row := range g {
		for x, c := range row {
			ret = append(ret, IndexedCell[T]{Idx: I(x, y), Cell: c})
		}
	}
	return ret
}

func (g Grid[T]) Indexes() []Index {
	var ret []Index
	for y, row := range g {
		for x := range row {
			ret = append(ret, I(x, y))
		}
	}
	return ret
}

func (g Grid[T]) Fill(t T) {
	for _, idx := range g.Indexes() {
		g.Set(idx, t)
	}
}

func (g Grid[T]) FilterValid(idxs []Index) []Index {
	var ret []Index
	for _, idx := range idxs {
		if _, ok := g.Lookup(idx); ok {
			ret = append(ret, idx)
		}
	}
	return ret
}

func NewGridSize[T any](cols, rows int) Grid[T] {
	var g Grid[T]
	for y := 0; y < rows; y++ {
		g = append(g, make([]T, cols))
	}
	return g
}

func FromGridSize[T, O any](other Grid[O]) Grid[T] {
	return NewGridSize[T](len(other[0]), len(other))
}

func Adjacent(i Index) []Index {
	var ret []Index
	for x := i.X - 1; x < i.X+2; x++ {
		for y := i.Y - 1; y < i.Y+2; y++ {
			if x == 0 && y == 0 {
				continue
			}
			ret = append(ret, Index{X: x, Y: y})
		}
	}
	return ret
}

func AdjacentNoDiagonal(i Index) []Index {
	return []Index{
		I(i.X, i.Y-1),
		I(i.X, i.Y+1),
		I(i.X-1, i.Y),
		I(i.X+1, i.Y),
	}
}

func Print[T rune | byte](g Grid[T]) {
	for _, row := range g {
		for _, e := range row {
			fmt.Printf("%c", e)
		}
		fmt.Println()
	}
}

func PrintFunc[T any](g Grid[T], f func(T) rune) {
	for _, row := range g {
		for _, e := range row {
			fmt.Printf("%c", f(e))
		}
		fmt.Println()
	}
}
