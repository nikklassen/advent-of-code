package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	f, _ := os.Open("input")
	scanner := bufio.NewScanner(f)
	edges := map[string][]string{}
	for scanner.Scan() {
		parts := strings.Split(scanner.Text(), " <-> ")
		adjacent := strings.Split(parts[1], ", ")
		edges[parts[0]] = adjacent
	}

	vertices := make(map[string]bool)
	for key, adjacent := range edges {
		vertices[key] = true
		for _, vertex := range adjacent {
			vertices[vertex] = true
		}
	}

	groups := 0
	for {
		next_group := ""
		for k := range vertices {
			delete(vertices, k)
			next_group = k
			break
		}
		if next_group == "" {
			break
		}
		groups++

		to_visit := []string{next_group}
		visited := make(map[string]bool)
		for {
			if len(to_visit) == 0 {
				break
			}

			current := to_visit[0]
			to_visit = to_visit[1:]

			visited[current] = true
			for _, vertex := range edges[current] {
				_, found := visited[vertex]
				if !found {
					visited[vertex] = true
					to_visit = append(to_visit, vertex)
					delete(vertices, vertex)
				}
			}
		}
	}
	fmt.Println(groups)
}
