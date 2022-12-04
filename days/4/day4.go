package _4

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"

	"github.com/ankon/adventofcode/2022/days"
)

//go:embed sample.txt
var sampleInput string

//go:embed input.txt
var fullInput string

func scanRange(s string) ([]int, error) {
	minS, maxS, found := strings.Cut(s, "-")
	if !found {
		return []int{}, fmt.Errorf("invalid range %q", s)
	}

	min, err := strconv.ParseInt(minS, 10, 0)
	if err != nil {
		return []int{}, err
	}
	max, err := strconv.ParseInt(maxS, 10, 0)
	if err != nil {
		return []int{}, err
	}
	return []int{int(min), int(max)}, nil
}

func rangeFullyContains(r1, r2 []int) bool {
	if r1[0] <= r2[0] && r1[1] >= r2[1] {
		return true
	}
	if r1[0] >= r2[0] && r1[1] <= r2[1] {
		return true
	}
	return false
}

func rangeOverlaps(r1, r2 []int) bool {
	if r1[0] <= r2[0] && r1[1] >= r2[0] {
		return true
	}
	if r1[0] <= r2[1] && r1[1] >= r2[1] {
		return true
	}
	if r2[0] <= r1[0] && r2[1] >= r1[1] {
		return true
	}
	if r2[0] <= r1[1] && r2[1] >= r1[1] {
		return true
	}
	return false
}

func Run(useSampleInput bool) error {
	input := days.PickInput(useSampleInput, sampleInput, fullInput)

	containedCount := 0
	overlapCount := 0
	for _, pair := range strings.Split(input, "\n") {
		if pair == "" {
			continue
		}
		r1, r2, found := strings.Cut(pair, ",")
		if !found {
			return fmt.Errorf("invalid pair definition %q", pair)
		}
		range1, err := scanRange(r1)
		if err != nil {
			return fmt.Errorf("cannot parse range %q: %w", r1, err)
		}
		range2, err := scanRange(r2)
		if err != nil {
			return fmt.Errorf("cannot parse range %q: %w", r2, err)
		}
		if rangeFullyContains(range1, range2) {
			containedCount++
		}
		if rangeOverlaps(range1, range2) {
			overlapCount++
		}
	}
	fmt.Printf("Pairs with fully contained ranges: %d\n", containedCount)
	fmt.Printf("Pairs with overlapping ranges: %d\n", overlapCount)

	return nil
}
