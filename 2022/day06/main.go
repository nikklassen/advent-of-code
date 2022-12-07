package main

import (
	_ "embed"
	"fmt"
	"strings"
)

const (
	startOfMessageLen = 14
	startOfPacketLen  = 4
)

//go:embed input.txt
var input string

func startOfPacket(cs []byte) bool {
	for i := 0; i < len(cs); i++ {
		for j := i + 1; j < len(cs); j++ {
			if cs[i] == cs[j] {
				return false
			}
		}
	}
	return true
}

func part1(input string) int {
	input = strings.TrimSpace(input)
	for i := startOfPacketLen; i < len(input); i++ {
		if startOfPacket([]byte(input[i-startOfPacketLen : i])) {
			return i
		}
	}
	panic("not found")
}

func startOfMessage(cs []byte) bool {
	m := map[byte]bool{}
	for _, c := range cs {
		if m[c] {
			return false
		}
		m[c] = true
	}
	return true
}

func part2(input string) int {
	input = strings.TrimSpace(input)
	for i := startOfMessageLen; i < len(input); i++ {
		if startOfMessage([]byte(input[i-startOfMessageLen : i])) {
			return i
		}
	}
	panic("not found")
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
