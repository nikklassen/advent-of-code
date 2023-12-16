package aocmaps

func Pop[M ~map[K]V, K comparable, V any](m M) (K, V, M) {
	var k K
	var v V
	for k, v = range m {
	}
	delete(m, k)
	return k, v, m
}

// Equals returns true if a and b have the same elements.
func Equals[M ~map[K]V, K, V comparable](a, b M) bool {
	if len(a) != len(b) {
		return false
	}
	for k, v1 := range a {
		if v2, ok := b[k]; !ok || v1 != v2 {
			return false
		}
	}
	return true
}

// Difference is a set that contains the elements of a that are not in b.
func Difference[M ~map[K]V, K, V comparable](a, b M) M {
	ret := M{}
	for k, v := range a {
		if _, ok := b[k]; !ok {
			ret[k] = v
		}
	}
	return ret
}
