//go:build ignore

package main

import (
	"bytes"
	"fmt"
	"os"
)

func main() {
	data := &bytes.Buffer{}

	fmt.Fprintln(data, `// Code generated by "generator.go"; DO NOT EDIT.`)
	fmt.Fprintln(data, "\npackage memo")
	fmt.Fprintln(data)

	for i := 1; i < 5; i++ {
		retType := "R"
		var inTypes string
		for j := 0; j < i; j++ {
			if j != 0 {
				inTypes += ", "
			}
			inTypes += fmt.Sprintf("T%d", j)
		}
		fmt.Fprintf(data, "type cacheKey%d[%s comparable] struct {\n", i, inTypes)
		for j := 0; j < i; j++ {
			fmt.Fprintf(data, "	v%[1]d T%[1]d\n", j)
		}
		fmt.Fprintln(data, "}")
		fmt.Fprintln(data)
		fmt.Fprintf(data, "func Memo%dx1", i)
		fmt.Fprintf(data, "[%[1]s comparable, %[2]s any](inner func(%[1]s) %[2]s) func(%[1]s) %[2]s {\n", inTypes, retType)
		fmt.Fprintf(data, "	c := map[cacheKey%d[%s]]%s{}\n", i, inTypes, retType)
		typeParams := ""
		for j := 0; j < i; j++ {
			if j != 0 {
				typeParams += ", "
			}
			typeParams += fmt.Sprintf("v%[1]d T%[1]d", j)
		}
		fmt.Fprintf(data, "	return func("+typeParams+") %s {\n", retType)
		values := ""
		for j := 0; j < i; j++ {
			if j != 0 {
				values += ", "
			}
			values += fmt.Sprintf("v%d", j)
		}
		fmt.Fprintf(data, "		key := cacheKey%d[%s]{%s}\n", i, inTypes, values)
		fmt.Fprintln(data, "		if v, ok := c[key]; ok {")
		fmt.Fprintln(data, "			return v")
		fmt.Fprintln(data, "		}")
		fmt.Fprintln(data, "		r := inner("+values+")")
		fmt.Fprintln(data, "		c[key] = r")
		fmt.Fprintln(data, "		return r")
		fmt.Fprintln(data, "	}")
		fmt.Fprintln(data, "}")
		fmt.Fprintln(data)
	}

	if err := os.WriteFile("memo_gen.go", data.Bytes(), 0o644); err != nil {
		fmt.Fprintf(os.Stderr, "Failed to write file: %v\n", err)
	}
}
