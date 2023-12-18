package pqueue

import (
	"container/heap"
)

type inner[T interface{ Less(other T) bool }] struct {
	items []T
}

func (in *inner[T]) Len() int { return len(in.items) }

func (in *inner[T]) Less(i, j int) bool { return in.items[i].Less(in.items[j]) }

func (in *inner[T]) Push(i any) {
	in.items = append(in.items, i.(T))
}

func (in *inner[T]) Pop() any {
	old := in.items[len(in.items)-1]
	var zero T
	in.items[len(in.items)-1] = zero
	in.items = in.items[:len(in.items)-1]
	return old
}

func (in *inner[T]) Swap(i, j int) {
	in.items[i], in.items[j] = in.items[j], in.items[i]
}

type PriorityQueue[T interface{ Less(other T) bool }] struct {
	inner inner[T]
}

func (pq *PriorityQueue[T]) Len() int { return pq.inner.Len() }

func (pq *PriorityQueue[T]) Push(x T) {
	heap.Push(&pq.inner, x)
}

func (pq *PriorityQueue[T]) Pop() T {
	return heap.Pop(&pq.inner).(T)
}

func (pq *PriorityQueue[T]) Remove(i int) T {
	return heap.Remove(&pq.inner, i).(T)
}
