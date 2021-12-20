package day18

import (
	"fmt"
	"io"
	"os"
	"path"
	"strconv"
	"strings"
)

var (
	// input = readSampleInputLines("day18")
	input = readInputLines("day18")
)

func readFileLines(dir string, fileName string) []string {
	file, _ := os.Open(path.Join(dir, fileName))
	contentBytes, _ := io.ReadAll(file)
	contents := strings.TrimSpace(string(contentBytes))
	return strings.Split(contents, "\n")
}

func readInputLines(dir string) []string {
	return readFileLines(dir, "input.txt")
}

func readSampleInputLines(dir string) []string {
	return readFileLines(dir, "sample_input.txt")
}

func parseInput() []*Tree {
	var ret []*Tree
	for _, line := range input {
		tree, _ := parseTree([]byte(line))
		ret = append(ret, tree)
	}
	return ret
}

type Leaf struct {
	value int
	prev  *Leaf
	next  *Leaf
}

func (l *Leaf) String() string {
	return fmt.Sprintf("Leaf(%d)", l.value)
}

type Tree struct {
	left  *Tree
	right *Tree

	leaf *Leaf
}

func (t *Tree) String() string {
	if t == nil {
		return ""
	}
	if t.leaf != nil {
		return strconv.FormatInt(int64(t.leaf.value), 10)
	}
	return fmt.Sprintf("[%s,%s]", t.left, t.right)
}

func cloneTree(tree *Tree) (*Tree, *Leaf, *Leaf) {
	if tree == nil {
		return nil, nil, nil
	}
	if tree.leaf != nil {
		leaf := &Leaf{value: tree.leaf.value}
		return &Tree{leaf: leaf}, leaf, leaf
	}
	left, leftLeftmost, leftRightmost := cloneTree(tree.left)
	right, rightLeftmost, rightRightmost := cloneTree(tree.right)
	newTree := &Tree{left: left, right: right}
	linkLeaves(leftRightmost, rightLeftmost)
	return newTree, leftLeftmost, rightRightmost
}

func (t *Tree) Clone() *Tree {
	newT, _, _ := cloneTree(t)
	return newT
}

func linkLeaves(prev, next *Leaf) {
	if prev != nil {
		prev.next = next
	}
	if next != nil {
		next.prev = prev
	}
}

func rightmostLeaf(tree *Tree) *Leaf {
	if tree == nil {
		return nil
	}
	rightmost := rightmostLeaf(tree.right)
	if rightmost != nil {
		return rightmost
	}
	if tree.leaf != nil {
		return tree.leaf
	}
	return rightmostLeaf(tree.left)
}

func leftmostLeaf(tree *Tree) *Leaf {
	if tree == nil {
		return nil
	}
	leftmost := leftmostLeaf(tree.left)
	if leftmost != nil {
		return leftmost
	}
	if tree.leaf != nil {
		return tree.leaf
	}
	return leftmostLeaf(tree.right)
}

func newTree(left *Tree, right *Tree) *Tree {
	t := &Tree{left: left, right: right}
	if left != nil && right != nil {
		linkLeaves(rightmostLeaf(left), leftmostLeaf(right))
	}
	return t
}

func leafTree(val int) *Tree {
	leaf := &Leaf{
		value: val,
	}
	return &Tree{
		leaf: leaf,
	}
}

func parseTree(s []byte) (*Tree, []byte) {
	if len(s) == 0 {
		return nil, s
	}
	if s[0] == '[' {
		left, s := parseTree(s[1:])
		right, s := parseTree(s[1:])
		return newTree(left, right), s[1:]
	} else if s[0] >= '0' && s[0] <= '9' {
		val := 0
		for len(s) > 0 && s[0] >= '0' && s[0] <= '9' {
			val *= 10
			val += int(s[0] - '0')
			s = s[1:]
		}
		return leafTree(val), s
	}
	return nil, s
}

func shouldExplode(tree *Tree, depth int) bool {
	return depth >= 4 && tree.left != nil && tree.left.leaf != nil && tree.right != nil && tree.right.leaf != nil
}

func explode(tree *Tree, depth int) bool {
	if tree == nil {
		return false
	}
	if shouldExplode(tree, depth) {
		newTree := *leafTree(0)
		newLeaf := newTree.leaf
		prev := tree.left.leaf.prev
		if prev != nil {
			prev.value += tree.left.leaf.value
			linkLeaves(prev, newLeaf)
		}
		next := tree.right.leaf.next
		if next != nil {
			next.value += tree.right.leaf.value
			linkLeaves(newLeaf, next)
		}
		*tree = newTree
		return true
	}
	return explode(tree.left, depth+1) || explode(tree.right, depth+1)
}

func split(tree *Tree) bool {
	if tree == nil {
		return false
	}
	if leaf := tree.leaf; leaf != nil {
		if v := leaf.value; v >= 10 {
			leftVal := v / 2
			rightVal := v - leftVal
			left := leafTree(leftVal)
			right := leafTree(rightVal)
			linkLeaves(left.leaf, right.leaf)
			*tree = Tree{left: left, right: right}
			linkLeaves(leaf.prev, tree.left.leaf)
			linkLeaves(tree.right.leaf, leaf.next)
			return true
		}
	}
	return split(tree.left) || split(tree.right)
}

func reduce(tree *Tree) {
	for {
		if explode(tree, 0) {
			continue
		}
		if split(tree) {
			continue
		}
		break
	}
}

func addTrees(lhs, rhs *Tree) *Tree {
	tree := newTree(lhs, rhs)
	reduce(tree)
	return tree
}

func magnitude(tree *Tree) int {
	if tree == nil {
		return 0
	}
	if tree.leaf != nil {
		return tree.leaf.value
	}
	return 3*magnitude(tree.left) + 2*magnitude(tree.right)
}

func sumTrees(trees []*Tree) *Tree {
	acc := trees[0]
	for _, tree := range trees[1:] {
		acc = addTrees(acc, tree)
	}
	return acc
}

func largestSum(trees []*Tree) int {
	maxMag := 0
	for i, t1 := range trees {
		for j, t2 := range trees {
			if i == j {
				continue
			}
			res := addTrees(t1.Clone(), t2.Clone())
			mag := magnitude(res)
			if mag > maxMag {
				maxMag = mag
			}
		}
	}
	return maxMag
}

func Part1() int {
	res := sumTrees(parseInput())
	return magnitude(res)
}

func Part2() int {
	return largestSum(parseInput())
}
