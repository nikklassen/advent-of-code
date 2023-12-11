package main

import (
	"flag"
	"fmt"
	"io"
	"net/http"
	"os"
	"strings"
)

var (
	year = flag.Int("year", 2023, "The year to generate a file for")
	day  = flag.Int("day", 0, "The day to generate a file for")
)

func downloadFile(w http.ResponseWriter, r *http.Request) {
	if r.Method != "POST" {
		http.Error(w, "Method Not Allowed", http.StatusMethodNotAllowed)
		return
	}
	data, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	fileName := fmt.Sprintf("%d/day%02d", *year, *day)
	isTestInput := strings.HasSuffix(r.URL.Path, "test")
	if isTestInput {
		fileName += "/test_input.txt"
	} else {
		fileName += "/input.txt"
	}
	if err := os.WriteFile(fileName, data, 0o644); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	if isTestInput {
		fmt.Printf("test ")
	}
	fmt.Println("input written")
}

func main() {
	flag.Parse()

	fmt.Println("Waiting for input files")

	http.HandleFunc("/", downloadFile)
	http.ListenAndServe(":8080", nil)
}
