//go:build ignore

package main

import "fmt"

func main() {
	fmt.Println("package memo")
	fmt.Println()

	for i := 1; i < 5; i++ {
		retType := "R"
		var inTypes string
		for j := 0; j < i; j++ {
			if j != 0 {
				inTypes += ","
			}
			inTypes += fmt.Sprintf("T%d", j)
		}
		typeParams := ""
		for j := 0; j < i; j++ {
			if j != 0 {
				typeParams += ", "
			}
			typeParams += fmt.Sprintf("v%[1]d T%[1]d", j)
		}
		fmt.Printf("type cacheKey%d struct {\n", i)
		fmt.Println("	" + typeParams)
		fmt.Println("}")
		fmt.Println()
		fmt.Printf("func Memo%dx1", i)
		fmt.Printf("[%[1]s comparable, %[2]s any](inner func(%[1]s) %[2]s) func(%[1]s) %[2]s {\n", inTypes, retType)
		fmt.Printf("	c := map[cacheKey%d]%s{}\n", i, retType)
		fmt.Printf("	return func("+typeParams+") %s {\n", retType)
		values := ""
		for j := 0; j < i; j++ {
			if j != 0 {
				values += ", "
			}
			values += fmt.Sprintf("v%d", j)
		}
		fmt.Println("		key := cacheKey{" + values + "}")
		fmt.Println("		if v, ok := c[key]; ok {")
		fmt.Println("			return v")
		fmt.Println("		}")
		fmt.Println("		r := inner(" + values + ")")
		fmt.Println("		c[key] = r")
		fmt.Println("		return r")
		fmt.Println("	}")
		fmt.Println("}")
		fmt.Println()
	}
}
