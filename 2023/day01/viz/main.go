//go:build js

package main

import (
	"fmt"
	"io"
	"net/http"
	"strings"
	"unicode"

	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
	"honnef.co/go/js/dom/v2"

	_ "embed"
)

var (
	digitWords = []string{
		"one",
		"two",
		"three",
		"four",
		"five",
		"six",
		"seven",
		"eight",
		"nine",
	}
)

func findDigit(line string, reverse, useWords bool) (start, end, value int) {
	i := 0
	inc := 1
	if reverse {
		i = len(line) - 1
		inc = -1
	}
	for {
		if useWords {
			for d, digit := range digitWords {
				if strings.HasPrefix(line[i:], digit) {
					return i, i + len(digit), d + 1
				}
			}
		}
		if unicode.IsDigit([]rune(line)[i]) {
			return i, i + 1, int(line[i] - '0')
		}
		i += inc
	}
}

func getValueAndHighlight(word string) (int, string) {
	sb := &strings.Builder{}
	firstStart, firstEnd, firstValue := findDigit(word, false, true)
	sb.WriteString(word[:firstStart])
	sb.WriteString("<span>")
	sb.WriteString(word[firstStart:firstEnd])
	sb.WriteString("</span>")

	lastStart, lastEnd, lastValue := findDigit(word, true, true)
	if lastStart != firstStart {
		sb.WriteString(word[firstEnd:lastStart])
		sb.WriteString("<span>")
		sb.WriteString(word[lastStart:lastEnd])
		sb.WriteString("</span>")
	}

	sb.WriteString(word[lastEnd:])

	num := firstValue*10 + lastValue
	fmt.Fprintf(sb, ": %d * 10 + %d = %d", firstValue, lastValue, num)
	return num, sb.String()
}

func readInput() (string, error) {
	resp, err := http.Get("/2023/day01/input.txt")
	if err != nil {
		return "", err
	}
	data, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", err
	}
	if err := resp.Body.Close(); err != nil {
		return "", err
	}
	return string(data), nil
}

func main() {
	document := dom.GetWindow().Document().(dom.HTMLDocument)

	body := document.Body()

	head := document.Head()
	style := document.CreateElement("style").(*dom.HTMLStyleElement)
	style.SetTextContent(`
span {
  color: red;
}`)
	head.AppendChild(style)

	d := document.QuerySelector("#content")

	input, err := readInput()
	if err != nil {
		panic(err.Error())
	}

	d.SetTextContent("")

	var tot int
	for _, line := range aocstrings.Lines(input) {
		s := document.CreateElement("div").(*dom.HTMLDivElement)

		value, highlighted := getValueAndHighlight(line)
		tot += value
		s.SetInnerHTML(highlighted)
		d.AppendChild(s)
	}

	body.InsertBefore(document.CreateTextNode(fmt.Sprintf("Total: %d", tot)), body.FirstChild())

	select {}
}
