package main

import (
	"embed"
	"errors"
	"flag"
	"fmt"
	"os"
	"path"
)

var (
	year = flag.String("year", "2023", "The year to generate a file for")
	day  = flag.Int("day", 0, "The day to generate a file for")

	//go:embed templates/*
	templateFiles embed.FS
)

func run(year string, day int) error {
	dir := path.Join(year, fmt.Sprintf("day%02d", day))
	if _, err := os.ReadDir(dir); errors.Is(err, os.ErrNotExist) {
		if err := os.Mkdir(dir, os.ModePerm); err != nil {
			return err
		}
	}

	templates, err := templateFiles.ReadDir("templates")
	if err != nil {
		return err
	}
	for _, t := range templates {
		contents, err := templateFiles.ReadFile(path.Join("templates", t.Name()))
		if err != nil {
			return err
		}
		destFile := path.Join(dir, t.Name())
		if err := os.WriteFile(destFile, contents, 0o644); err != nil {
			return err
		}
	}
	return nil
}

func main() {
	flag.Parse()
	if err := run(*year, *day); err != nil {
		fmt.Fprintln(os.Stderr, err.Error())
		os.Exit(1)
	}
}
