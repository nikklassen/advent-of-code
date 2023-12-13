package main

import (
	"embed"
	"errors"
	"flag"
	"fmt"
	"os"
	"path"
	"path/filepath"
)

var (
	year = flag.String("year", "2023", "The year to generate a file for")
	day  = flag.Int("day", 0, "The day to generate a file for")
	lang = flag.String("lang", "go", "The language to generate a skeleton for")

	//go:embed templates/*
	templateFiles embed.FS
)

func run(year string, day int, lang string) error {
	dir := path.Join(year, fmt.Sprintf("day%02d", day))
	if _, err := os.ReadDir(dir); errors.Is(err, os.ErrNotExist) {
		if err := os.Mkdir(dir, os.ModePerm); err != nil {
			return err
		}
	}

	langTemplates := filepath.Join("templates", lang)
	templates, err := templateFiles.ReadDir(langTemplates)
	if err != nil {
		return err
	}
	for _, t := range templates {
		contents, err := templateFiles.ReadFile(filepath.Join(langTemplates, t.Name()))
		if err != nil {
			return err
		}
		destFile := path.Join(dir, t.Name())
		if err := os.WriteFile(destFile, contents, 0o644); err != nil {
			return err
		}
	}
	fmt.Printf("wrote %s skeleton to %s\n", lang, dir)
	return nil
}

func main() {
	flag.Parse()
	if err := run(*year, *day, *lang); err != nil {
		fmt.Fprintln(os.Stderr, err.Error())
		os.Exit(1)
	}
}
