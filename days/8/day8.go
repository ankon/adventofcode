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

func visibilityInDirection(trees trees, x, y int, d direction) (int, bool) {
	height := len(trees)
	width := len(trees[0])

	result := 0
	t := trees[y][x]
	x += d.dx
	y += d.dy
	for x >= 0 && x < width && y >= 0 && y < height {
		result++
		if trees[y][x] >= t {
			return result, false
		}
		x += d.dx
		y += d.dy
	}
	return result, true
}

func isVisibleFromOutside(trees trees, x, y int) bool {
	for _, d := range directions {
		if _, visibleFromOutside := visibilityInDirection(trees, x, y, d); visibleFromOutside {
			return true
		}
	}
	return false
}

func calculateScenicScore(trees trees, x, y int) int {
	result := 1
	for _, d := range directions {
		distance, _ := visibilityInDirection(trees, x, y, d)
		result *= distance
	}
	return result
}

func printScenicScoreMap(scenicScoreMap [][]int) {
	for y := 0; y < len(scenicScoreMap); y++ {
		s := ""
		for x := 0; x < len(scenicScoreMap[y]); x++ {
			s += fmt.Sprintf("%4d ", scenicScoreMap[y][x])
		}
		fmt.Println(s)
	}
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
			if isVisibleFromOutside(trees, x, y) {
				count++
			}
		}
	}
	fmt.Printf("%d visible trees\n", count)

	scenicScoreMap := make([][]int, height)
	highestScenicScore := 0
	for y := 0; y < height; y++ {
		scenicScoreMap[y] = make([]int, width)
		for x := 0; x < width; x++ {
			scenicScore := calculateScenicScore(trees, x, y)
			scenicScoreMap[y][x] = scenicScore
			if scenicScore > highestScenicScore {
				highestScenicScore = scenicScore
			}
		}
	}
	fmt.Printf("highest scenic score %d\n", highestScenicScore)
	printScenicScoreMap(scenicScoreMap)

	return nil
}
