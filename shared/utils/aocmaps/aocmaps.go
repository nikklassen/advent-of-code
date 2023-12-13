package aocmaps

func Pop[K comparable, V any](m map[K]V) (K, V, map[K]V) {
	var k K
	var v V
	for k, v = range m {
	}
	delete(m, k)
	return k, v, m
}
