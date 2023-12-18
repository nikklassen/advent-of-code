package grid

var (
	Up    = Index{0, -1}
	Down  = Index{0, 1}
	Left  = Index{-1, 0}
	Right = Index{1, 0}
)

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

func FlipDir(dir Index) Index {
	return Index{
		X: dir.X * -1,
		Y: dir.Y * -1,
	}
}

func I(x, y int) Index {
	return Index{X: x, Y: y}
}
