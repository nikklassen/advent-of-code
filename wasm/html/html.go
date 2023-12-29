//go:build js

package html

import (
	"syscall/js"

	"honnef.co/go/js/dom/v2"
)

type Parser struct {
	r js.Value
}

func NewParser() *Parser {
	document := js.Global().Get("document")
	r := document.Call("createRange")
	r.Call("selectNode", document.Get("body"))
	return &Parser{r}
}

func (p *Parser) Parse(html string) dom.Element {
	fragment := p.r.Call("createContextualFragment", js.ValueOf(html))
	return dom.WrapElement(fragment.Get("firstChild"))
}
