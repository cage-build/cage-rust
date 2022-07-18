package main

import (
	"fmt"
	"os"
	"time"
)

func main() {
	f, err := os.Create("time.txt")
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	defer f.Close()

	for ti := time.Unix(0, 12_345_678).UTC(); ti.Year() < 1970+401; ti = ti.Add(time.Hour * 24) {
		fmt.Fprintf(f, "%d\t%s\n", ti.Unix(), ti.Format(time.RFC3339Nano))
	}
}
