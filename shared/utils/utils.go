package utils

import (
	"slices"

	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
)

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

type Range struct {
	Start, End int
}

func (r Range) Len() int {
	return r.End - r.Start
}

func (r Range) Contains(i int) bool {
	return i >= r.Start && i < r.End
}

func (r Range) Intersect(other Range) Range {
	if other.Start < r.Start {
		r, other = other, r
	}
	if r.End <= other.Start {
		return Range{}
	}
	return Range{
		Start: max(r.Start, other.Start),
		End:   min(r.End, other.End),
	}
}

func (r Range) Union(other Range) RangeSet {
	if other.Start < r.Start {
		r, other = other, r
	}
	if r.End < other.Start {
		return RangeSet{{r.Start, r.End}, {other.Start, other.End}}
	}
	return RangeSet{{r.Start, max(r.End, other.End)}}
}

func (r Range) Difference(other Range) RangeSet {
	if r.Start > other.End || r.End < other.Start {
		return RangeSet{r}
	}
	var rs RangeSet
	if other.Start > r.Start {
		rs = append(rs, Range{r.Start, other.Start})
	}
	if other.End < r.End {
		rs = append(rs, Range{r.End, other.End})
	}
	return rs
}

type RangeSet []Range

func (rs RangeSet) Len() int {
	return aocslices.Sum(aocslices.Map(rs, Range.Len))
}

func (rs RangeSet) Intersect(other RangeSet) RangeSet {
	var ret RangeSet
	for _, a := range rs {
		for _, b := range other {
			ma := max(a.Start, b.Start)
			mi := min(a.End, b.End)
			if ma < mi {
				ret = append(ret, Range{Start: ma, End: mi})
			}
		}
	}
	return ret
}

func (rs RangeSet) Union(other RangeSet) RangeSet {
	sorted := slices.Clone(rs)
	sorted = append(sorted, other...)
	slices.SortFunc(sorted, func(a, b Range) int {
		return a.Start - b.Start
	})

	var ret RangeSet
	for len(sorted) > 1 {
		parts := sorted[0].Union(sorted[1])
		sorted = sorted[1:]
		if len(parts) == 2 {
			ret = append(ret, parts[0])
			sorted[0] = parts[1]
		} else {
			sorted[0] = parts[0]
		}
	}
	ret = append(ret, sorted...)
	return ret
}

func (rs RangeSet) Difference(other RangeSet) RangeSet {
	ret := rs
	for _, j := range other {
		var newRet RangeSet
		for _, i := range ret {
			newRet = append(newRet, i.Difference(j)...)
		}
		ret = newRet
	}
	return ret
}
