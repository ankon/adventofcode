package _8

import (
	_ "embed"
	"fmt"
	"strings"

	"github.com/ankon/adventofcode/2022/days"
)

//go:embed sample.txt
var sampleInput string

//go:embed input.txt
var fullInput string

type trees [][]byte

type direction struct {
	dx, dy int
}

var directions = []direction{
	{0, -1},
	{1, 0},
	{0, 1},
	{-1, 0},
}

func isVisibleInDirection(trees trees, x, y int, d direction) bool {
	height := len(trees)
	width := len(trees[0])

	t := trees[y][x]
	x += d.dx
	y += d.dy
	for x >= 0 && x < width && y >= 0 && y < height {
		if trees[y][x] >= t {
			return false
		}
		x += d.dx
		y += d.dy
	}
	return true
}

func isVisible(trees trees, x, y int) bool {
	for _, d := range directions {
		if isVisibleInDirection(trees, x, y, d) {
			return true
		}
	}
	return false
}

func Run(useSampleInput bool) error {
	input := days.PickInput(useSampleInput, sampleInput, fullInput)
	lines := strings.Split(strings.TrimSpace(input), "\n")
	trees := make([][]byte, 0, len(lines))
	for _, line := range lines {
		trees = append(trees, []byte(line))
	}
	height := len(trees)
	width := len(trees[0])

	count := 0
	for y := 0; y < height; y++ {
		for x := 0; x < width; x++ {
			if isVisible(trees, x, y) {
				count++
			}
		}
	}
	fmt.Printf("%d visible trees\n", count)

	return nil
}
