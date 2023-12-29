//go:build js

package main

import (
	"fmt"
	"io"
	"net/http"
	"strings"
	"time"

	"github.com/nikklassen/advent-of-code/shared/utils/aocslices"
	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
	"github.com/nikklassen/advent-of-code/wasm/animate"
	"github.com/nikklassen/advent-of-code/wasm/html"
	"honnef.co/go/js/dom/v2"

	_ "embed"
)

var (
	//go:embed bag.svg
	bagSVG string
	//go:embed ball.svg
	ballSVG string
)

func readInput() (string, error) {
	resp, err := http.Get("/2023/day02/input.txt")
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

type game struct {
	draws    [][]string
	minBalls map[string]int
}

func parseGames(input string) []game {
	var ret []game
	for _, line := range aocstrings.Lines(input) {
		_, sets, _ := strings.Cut(line, ": ")
		g := game{
			minBalls: map[string]int{},
		}
		for _, set := range strings.Split(sets, "; ") {
			var draw []string
			for _, balls := range strings.Split(set, ", ") {
				countStr, colour, _ := strings.Cut(balls, " ")
				count := aocstrings.MustAtoi(countStr)
				g.minBalls[colour] = max(g.minBalls[colour], count)
				draw = append(draw, aocslices.Repeat(colour, count)...)
			}
			g.draws = append(g.draws, draw)
		}
		ret = append(ret, g)
	}
	return ret
}

type point struct {
	x, y float64
}

func moveElement(complete float64, e dom.HTMLElement, start, end point) {
	x := (end.x - start.x) * complete
	y := (end.y - start.y) * complete
	e.Style().SetProperty("transform", fmt.Sprintf("translate(%.2fpx, %.2fpx)", x, y), "")
}

type slideAnimation struct {
	e          *dom.BasicHTMLElement
	start, end point
}

func drawBalls(ballBox, ballTemplate dom.Element, bag point, draw []string) []slideAnimation {
	ballBox.SetInnerHTML("")

	y := 75.0

	var animations []slideAnimation
	var redIdx, greenIdx, blueIdx float64
	for _, ballColour := range draw {
		ball1 := ballTemplate.CloneNode(true).(*dom.BasicHTMLElement)
		ball1.QuerySelector("circle").SetAttribute("fill", ballColour)
		ball1.SetAttribute("width", "50")
		ball1.SetAttribute("height", "50")
		ball1.SetAttribute("style", fmt.Sprintf("position: absolute; top: %.2fpx; left: %.2fpx", bag.y, bag.x))
		ballBox.AppendChild(ball1)
		var end point
		switch ballColour {
		case "red":
			end = point{100 + 75*redIdx, y}
			redIdx++
		case "green":
			end = point{100 + 75*greenIdx, y + 75}
			greenIdx++
		case "blue":
			end = point{100 + 75*blueIdx, y + 150}
			blueIdx++
		}
		animations = append(animations, slideAnimation{
			e:     ball1,
			start: point{bag.x, bag.y},
			end:   end,
		})
	}
	return animations
}

type animationState struct {
	games                []game
	gameIndex, drawIndex int
	animations           []slideAnimation

	h1, ballBox, ballTemplate dom.Element
	bagCenter                 point
}

func (s *animationState) ready() bool {
	return len(s.animations) == 0
}

func (s *animationState) nextDraw() {
	if s.drawIndex == len(s.games[s.gameIndex].draws) {
		s.h1.SetTextContent(fmt.Sprintf("Game %d, Result", s.gameIndex+1))
		var balls []string
		fmt.Println(s.games[s.gameIndex].minBalls)
		for colour, count := range s.games[s.gameIndex].minBalls {
			balls = append(balls, aocslices.Repeat(colour, count)...)
		}
		s.animations = drawBalls(s.ballBox, s.ballTemplate, s.bagCenter, balls)
		s.gameIndex++
		s.drawIndex = 0
	} else {
		s.h1.SetTextContent(fmt.Sprintf("Game %d, Draw %d", s.gameIndex+1, s.drawIndex+1))
		s.animations = drawBalls(s.ballBox, s.ballTemplate, s.bagCenter, s.games[s.gameIndex].draws[s.drawIndex])
		s.drawIndex++
	}

	done := animate.Animate(dom.GetWindow(), 400*time.Millisecond, func(complete float64) {
		for _, a := range s.animations {
			moveElement(complete, a.e, a.start, a.end)
		}
	})
	<-done
	s.animations = nil
}

func main() {
	document := dom.GetWindow().Document().(dom.HTMLDocument)

	head := document.Head()
	style := document.CreateElement("style").(*dom.HTMLStyleElement)
	style.SetTextContent(`
span {
  color: red;
}`)
	head.AppendChild(style)

	d := document.QuerySelector("#content")
	h1 := document.CreateElement("h1")
	d.AppendChild(h1)

	input, err := readInput()
	if err != nil {
		panic(err.Error())
	}

	parser := html.NewParser()
	svg := parser.Parse(bagSVG)
	svg.SetAttribute("style", "position: absolute; top: 400px; left: 200px; z-index: 1")
	d.AppendChild(svg)

	r := svg.GetBoundingClientRect()
	x := r.Left() + (r.Width() / 2) - 25
	y := r.Top() + (r.Height() / 2) - 25

	ballTemplate := parser.Parse(ballSVG).(*dom.BasicHTMLElement)

	ballBox := document.CreateElement("div")
	d.AppendChild(ballBox)

	games := parseGames(input)

	state := &animationState{
		games: games,

		h1:           h1,
		ballBox:      ballBox,
		ballTemplate: ballTemplate,
		bagCenter:    point{x, y},
	}

	svg.AddEventListener("click", false, func(e dom.Event) {
		e.PreventDefault()
		if !state.ready() {
			return
		}
		go state.nextDraw()
	})

	select {}
}
