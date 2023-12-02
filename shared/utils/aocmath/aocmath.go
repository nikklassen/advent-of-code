package aocmath

import (
	"math"

	"golang.org/x/exp/constraints"
)

func Abs[E constraints.Integer | constraints.Float](x E) E {
	return E(math.Abs(float64(x)))
}

func Max[E constraints.Integer | constraints.Float](x, y E) E {
	return E(math.Max(float64(x), float64(y)))
}

func Min[E constraints.Integer | constraints.Float](x, y E) E {
	return E(math.Min(float64(x), float64(y)))
}

func Sign[E constraints.Signed | constraints.Float](x E) E {
	if x < 0 {
		return E(-1)
	}
	return E(1)
}
