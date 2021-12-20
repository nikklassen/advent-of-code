package day18

import "testing"

func TestExplode(t *testing.T) {
	tests := []struct {
		init string
		want string
	}{
		{"[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"},
		{"[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"},
		{"[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"},
		{
			"[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
			"[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
		},
	}
	for _, test := range tests {
		got, _ := parseTree(test.init)
		explode(got, 0)
		if got.String() != test.want {
			t.Errorf("explode(%s) got %s, want %s", test.init, got, test.want)
		}
	}
}

func TestSplit(t *testing.T) {
	tests := []struct {
		init string
		want string
	}{
		{"[10,1]", "[[5,5],1]"},
		{"[11,1]", "[[5,6],1]"},
		{"[12,13]", "[[6,6],13]"},
	}
	for _, test := range tests {
		got, _ := parseTree(test.init)
		split(got)
		if got.String() != test.want {
			t.Errorf("split(%s) got %s, want %s", test.init, got, test.want)
		}
	}
}

func TestNewTree(t *testing.T) {
	n1, _ := parseTree("[1,2]")
	n2, _ := parseTree("[3,4]")
	got := newTree(n1, n2)
	want := "[[1,2],[3,4]]"
	if got.String() != want {
		t.Errorf("addTrees(%s, %s) got %s, want %s", n1, n2, got, want)
	}
}

func TestAdd(t *testing.T) {
	tests := []struct {
		lhs, rhs string
		want     string
	}{
		{
			lhs:  "[[[[4,3],4],4],[7,[[8,4],9]]]",
			rhs:  "[1,1]",
			want: "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
		},
		{
			lhs:  "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
			rhs:  "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
			want: "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
		},
	}
	for _, test := range tests {
		n1, _ := parseTree(test.lhs)
		n2, _ := parseTree(test.rhs)
		got := addTrees(n1, n2)
		if got.String() != test.want {
			t.Errorf("addTrees(%s, %s) got %s, want %s", n1, n2, got, test.want)
		}
	}
}

func TestAddList(t *testing.T) {
	input := []string{
		"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
		"[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
		"[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
		"[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
		"[7,[5,[[3,8],[1,4]]]]",
		"[[2,[2,2]],[8,[8,1]]]",
		"[2,9]",
		"[1,[[[9,3],9],[[9,0],[0,7]]]]",
		"[[[5,[7,4]],7],1]",
		"[[[[4,2],2],6],[8,7]]",
	}
	var trees []*Tree
	for _, line := range input {
		tree, _ := parseTree(line)
		trees = append(trees, tree)
	}
	got := sumTrees(trees)
	want := "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
	if got.String() != want {
		t.Errorf("sumTrees(_) got %s, want %s", got, want)
	}
}

func TestMagnitude(t *testing.T) {
	tests := []struct {
		tree string
		want int
	}{
		{"[[1,2],[[3,4],5]]", 143},
		{"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384},
		{"[[[[1,1],[2,2]],[3,3]],[4,4]]", 445},
		{"[[[[3,0],[5,3]],[4,4]],[5,5]]", 791},
		{"[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137},
		{
			"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
			3488,
		},
	}
	for _, test := range tests {
		tree, _ := parseTree(test.tree)
		got := magnitude(tree)
		if got != test.want {
			t.Errorf("magnitude(%s) got %d, want %d", tree, got, test.want)
		}
	}
}
