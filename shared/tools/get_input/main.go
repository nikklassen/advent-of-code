package main

import (
	"context"
	"errors"
	"fmt"
	"io"
	"net/http"
	"os"
	"os/signal"

	"github.com/nikklassen/advent-of-code/shared/utils/aocstrings"
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
	path := r.URL.Query().Get("path")
	reqURL, isInput := aocstrings.TryTrimSuffix(path, "/input")
	var year, day int
	if _, err := fmt.Sscanf(reqURL, "https://adventofcode.com/%d/day/%d", &year, &day); err != nil {
		fmt.Fprintln(os.Stderr, "Failed to extract day/year:", err.Error())
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	fileName := fmt.Sprintf("%d/day%02d", year, day)
	if isInput {
		fileName += "/input.txt"
	} else {
		fileName += "/test_input.txt"
	}
	if err := os.WriteFile(fileName, data, 0o644); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	if !isInput {
		fmt.Printf("test ")
	}
	fmt.Println("input written to", fileName)
}

func main() {
	ctx := context.Background()

	fmt.Println("Waiting for files")

	http.HandleFunc("/", downloadFile)
	go func() {
		if err := http.ListenAndServe(":8080", nil); !errors.Is(err, http.ErrServerClosed) {
			fmt.Fprintln(os.Stderr, "Server exited with an error:", err.Error())
		}
	}()
	ctx, cancel := signal.NotifyContext(ctx, os.Interrupt)
	defer cancel()

	<-ctx.Done()
}
