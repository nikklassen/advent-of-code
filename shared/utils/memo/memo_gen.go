// Code generated by "generator.go"; DO NOT EDIT.

package memo

type cacheKey1[T0 comparable] struct {
	v0 T0
}

func Memo1x1[T0 comparable, R any](inner func(T0) R) func(T0) R {
	c := map[cacheKey1[T0]]R{}
	return func(v0 T0) R {
		key := cacheKey1[T0]{v0}
		if v, ok := c[key]; ok {
			return v
		}
		r := inner(v0)
		c[key] = r
		return r
	}
}

type cacheKey2[T0, T1 comparable] struct {
	v0 T0
	v1 T1
}

func Memo2x1[T0, T1 comparable, R any](inner func(T0, T1) R) func(T0, T1) R {
	c := map[cacheKey2[T0, T1]]R{}
	return func(v0 T0, v1 T1) R {
		key := cacheKey2[T0, T1]{v0, v1}
		if v, ok := c[key]; ok {
			return v
		}
		r := inner(v0, v1)
		c[key] = r
		return r
	}
}

type cacheKey3[T0, T1, T2 comparable] struct {
	v0 T0
	v1 T1
	v2 T2
}

func Memo3x1[T0, T1, T2 comparable, R any](inner func(T0, T1, T2) R) func(T0, T1, T2) R {
	c := map[cacheKey3[T0, T1, T2]]R{}
	return func(v0 T0, v1 T1, v2 T2) R {
		key := cacheKey3[T0, T1, T2]{v0, v1, v2}
		if v, ok := c[key]; ok {
			return v
		}
		r := inner(v0, v1, v2)
		c[key] = r
		return r
	}
}

type cacheKey4[T0, T1, T2, T3 comparable] struct {
	v0 T0
	v1 T1
	v2 T2
	v3 T3
}

func Memo4x1[T0, T1, T2, T3 comparable, R any](inner func(T0, T1, T2, T3) R) func(T0, T1, T2, T3) R {
	c := map[cacheKey4[T0, T1, T2, T3]]R{}
	return func(v0 T0, v1 T1, v2 T2, v3 T3) R {
		key := cacheKey4[T0, T1, T2, T3]{v0, v1, v2, v3}
		if v, ok := c[key]; ok {
			return v
		}
		r := inner(v0, v1, v2, v3)
		c[key] = r
		return r
	}
}

