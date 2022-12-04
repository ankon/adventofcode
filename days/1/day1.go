package _1

import (
	_ "embed"
	"fmt"
	"sort"
	"strconv"
	"strings"

	"github.com/ankon/adventofcode/2022/days"
)

//go:embed sample.txt
var sampleInput string

//go:embed input.txt
var fullInput string

type topN struct {
	data []int
}

func makeTopN(n int) topN {
	data := make([]int, 0, n)
	return topN{data}
}

// Insert inserts the given value if it fits (i.e. either there is capacity left, or it is bigger than at least one existing value).
func (t *topN) Insert(v int) bool {
	l := len(t.data)

	// Find the insert point
	at := sort.Search(l, func(i int) bool { return v >= t.data[i] })

	// No space to add it, and not big enough to shift something out.
	if at == cap(t.data) {
		return false
	}

	// We might not be using all of the capacity yet, expand the array
	dropAtEnd := 1
	if l < cap(t.data) {
		t.data = append(t.data, 0)
		dropAtEnd = 0
	}

	// Shift things if the insert point is not at the end
	shiftLen := l - at - dropAtEnd
	if shiftLen > 0 {
		copy(t.data[at+1:at+1+shiftLen], t.data[at:at+shiftLen])
	}
	t.data[at] = v
	return true
}

func (t *topN) Iterate() []int {
	return t.data
}

func findHighestCaloriesOnElf(input string, n int) (int, error) {
	topN := makeTopN(n)

	var current int
	for _, line := range strings.Split(input, "\n") {
		if line == "" {
			topN.Insert(current)
			current = 0
		} else {
			calories, err := strconv.ParseInt(line, 0, 0)
			if err != nil {
				return 0, err
			}
			current += int(calories)
		}
	}

	result := 0
	for _, calories := range topN.Iterate() {
		result += calories
	}
	return result, nil
}


func Run(useSampleInput bool) error {
	input := days.PickInput(useSampleInput, sampleInput, fullInput)

	highest, err := findHighestCaloriesOnElf(input, 1)
	if err != nil {
		return err
	}
	fmt.Printf("Highest calorie count on an elf: %d\n", highest)

	highest3, err := findHighestCaloriesOnElf(input, 3)
	if err != nil {
		return err
	}
	fmt.Printf("Sum of the 3 highest calorie counts on an elf: %d\n", highest3)
	
	return nil
}
