package pqueue

import (
	"math/rand"
	"slices"
	"testing"
)

type pqInt int

func (i pqInt) Less(j pqInt) bool {
	return i < j
}

func FuzzPriorityQueue(f *testing.F) {
	f.Fuzz(func(t *testing.T, seed int64, n uint) {
		r := rand.New(rand.NewSource(seed))
		pq := PriorityQueue[pqInt]{}
		for i := 0; i < int(n); i++ {
			pq.Push(pqInt(r.Int()))
		}
		if pq.Len() != int(n) {
			t.Fatalf("Len() = %d, want %d", pq.Len(), n)
		}
		var items []int
		for pq.Len() > 0 {
			items = append(items, int(pq.Pop()))
		}
		if !slices.IsSorted(items) {
			t.Fatal("items are not sorted")
		}
	})
}
