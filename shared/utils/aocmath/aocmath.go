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

func Pow[E constraints.Integer | constraints.Float](x, y E) E {
	return E(math.Pow(float64(x), float64(y)))
}

func GCD[E constraints.Unsigned](x, y E) E {
	var tmp E
	for x > 0 {
		tmp = x
		x = y % x
		y = tmp
	}
	return y
}

func LCM[E constraints.Unsigned](x, y E) E {
	return x / GCD(x, y) * y
}

func LCMAll[E constraints.Unsigned](xs []E) E {
	lcmFactors := map[E]E{}
	for _, x := range xs {
		factors := PrimeFactors(x)
		groups := map[E]E{}
		for _, f := range factors {
			groups[f]++
		}
		for n, i := range groups {
			lcmFactors[n] = max(lcmFactors[n], i)
		}
	}
	ret := E(1)
	for n, i := range lcmFactors {
		ret *= Pow(n, i)
	}
	return ret
}

func PrimeFactors[E constraints.Unsigned](x E) []E {
	var ret []E
	for i := E(2); i <= x; i++ {
		div := x / i
		mod := x % i
		for mod == 0 {
			ret = append(ret, i)
			x = div
			div = x / i
			mod = x % i
		}
	}
	return ret
}
