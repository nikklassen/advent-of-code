package main

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"

	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

//go:embed input.txt
var input string

type file struct {
	name string
	size int
}

type directory struct {
	parent         *directory
	name           string
	files          []file
	subDirectories []*directory

	size int
}

func cd(d *directory, path string) *directory {
	if path == ".." {
		return d.parent
	}
	for _, sd := range d.subDirectories {
		if sd.name == path {
			return sd
		}
	}
	panic(fmt.Sprintf("not found: %q", path))
}

func buildDir(dir *directory, lines []string) int {
	n := 0
	for i := 0; i < len(lines); i++ {
		s := lines[i]
		if s[0] == '$' {
			break
		}
		n++
		if path, ok := aocstrings.TryTrimPrefix(s, "dir "); ok {
			sd := &directory{
				name:   path,
				parent: dir,
			}
			dir.subDirectories = append(dir.subDirectories, sd)
			continue
		}
		sizeStr, name, _ := strings.Cut(s, " ")
		size, err := strconv.Atoi(sizeStr)
		if err != nil {
			panic(err)
		}
		dir.files = append(dir.files, file{
			name: name,
			size: size,
		})
	}
	return n
}

func computeSizes(dir *directory) int {
	size := 0
	for _, sd := range dir.subDirectories {
		size += computeSizes(sd)
	}
	for _, f := range dir.files {
		size += f.size
	}
	dir.size = size
	return size
}

func findSmallDirs(dir *directory) []*directory {
	var found []*directory
	if dir.size < 100_000 {
		found = append(found, dir)
	}
	for _, sd := range dir.subDirectories {
		found = append(found, findSmallDirs(sd)...)
	}
	return found
}

func print(dir *directory, level int) {
	prefix := strings.Repeat(" ", 2*level)
	fmt.Printf("%s- %s (dir)\n", prefix, dir.name)
	for _, sd := range dir.subDirectories {
		print(sd, level+1)
	}
	for _, f := range dir.files {
		fmt.Printf("%s  - %s (file, size=%d)\n", prefix, f.name, f.size)
	}
}

func newFS(input string) *directory {
	root := &directory{name: "/"}
	dir := root
	lines := aocstrings.Lines(input)[1:]
	i := 0
	for ; i < len(lines); i++ {
		s := lines[i]
		if path, ok := aocstrings.TryTrimPrefix(s, "$ cd "); ok {
			dir = cd(dir, path)
			continue
		}
		if strings.HasPrefix(s, "$ ls") {
			n := buildDir(dir, lines[i+1:])
			i += n
			continue
		}
		panic(fmt.Sprintf("unexpected line: %q", s))
	}
	computeSizes(root)
	return root
}

func part1(input string) int {
	root := newFS(input)
	found := findSmallDirs(root)
	return aocslices.Sum(aocslices.Map(found, func(d *directory) int { return d.size }))
}

func findSmallest(d *directory, toDelete, smallest int) int {
	if d.size > toDelete && d.size < smallest {
		smallest = d.size
	}
	for _, sd := range d.subDirectories {
		smallest = findSmallest(sd, toDelete, smallest)
	}
	return smallest
}

func part2(input string) int {
	root := newFS(input)
	toDelete := 30_000_000 - (70_000_000 - root.size)
	return findSmallest(root, toDelete, root.size)
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
