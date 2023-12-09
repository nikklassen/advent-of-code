package main

import (
	_ "embed"
	"fmt"
	"slices"
	"strings"

	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
)

var (
	//go:embed input.txt
	input string
)

type handType int

const (
	fiveOfAKind handType = iota + 1
	fourOfAKind
	fullHouse
	threeOfAKind
	twoPair
	onePair
	highCard
)

const (
	jokerGroup = 6
)

var (
	normalRanks = []rune{'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'}
	jokerRanks  = []rune{'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'}
)

func compareHands(a, b []rune, ranks []rune) int {
	for i, cardA := range a {
		cardB := b[i]
		if cardA != cardB {
			return slices.Index(ranks, cardA) - slices.Index(ranks, cardB)
		}
	}
	return 0
}

type handBid struct {
	hand        []rune
	groupCounts map[int]int
	handType    handType
	bid         int
}

func sortHandBids(handBids []handBid, ranks []rune) {
	slices.SortFunc(handBids, func(a, b handBid) int {
		if a.handType != b.handType {
			return int(a.handType) - int(b.handType)
		}
		return compareHands(a.hand, b.hand, ranks)
	})
}

func groupByCardCount(hand []rune, useJokers bool) map[int]int {
	var jokers int
	groups := map[rune]int{}
	for _, card := range hand {
		if useJokers && card == 'J' {
			jokers++
			continue
		}
		groups[card]++
	}
	ret := map[int]int{}
	for _, g := range groups {
		ret[g]++
	}
	ret[jokerGroup] = jokers
	return ret
}

func typeOfHand(hand map[int]int) handType {
	switch {
	case hand[5] == 1:
		return fiveOfAKind
	case hand[4] == 1:
		return fourOfAKind
	case hand[3] == 1 && hand[2] == 1:
		return fullHouse
	case hand[3] == 1:
		return threeOfAKind
	case hand[2] == 2:
		return twoPair
	case hand[2] == 1:
		return onePair
	default:
		return highCard
	}
}

func totalBids(handBids []handBid) int {
	var ret int
	for i, hb := range handBids {
		ret += (len(handBids) - i) * hb.bid
	}
	return ret
}

func computeTotalPoints(input string, useJokers bool) int {
	lines := aocstrings.Lines(input)
	var handBids []handBid
	for _, line := range lines {
		hand, bid, _ := strings.Cut(line, " ")
		handCards := []rune(hand)
		groupCounts := groupByCardCount(handCards, useJokers)
		ht := typeOfHand(groupCounts)
		if useJokers {
			for i := 0; i < groupCounts[jokerGroup]; i++ {
				ht = addJoker(ht)
			}
		}
		handBids = append(handBids, handBid{
			hand:        handCards,
			bid:         aocstrings.MustAtoi(bid),
			groupCounts: groupCounts,
			handType:    ht,
		})
	}
	ranks := normalRanks
	if useJokers {
		ranks = jokerRanks
	}
	sortHandBids(handBids, ranks)
	return totalBids(handBids)
}

func part1(input string) int {
	return computeTotalPoints(input, false)
}

func addJoker(ht handType) handType {
	switch ht {
	case fiveOfAKind, fourOfAKind:
		return fiveOfAKind
	case fullHouse:
		panic("can't add a joker to a full house")
	case threeOfAKind:
		return fourOfAKind
	case twoPair:
		return fullHouse
	case onePair:
		return threeOfAKind
	default:
		return onePair
	}
}

func part2(input string) int {
	return computeTotalPoints(input, true)
}

func main() {
	fmt.Printf("part 1: %d\n", part1(input))
	fmt.Printf("part 2: %d\n", part2(input))
}
