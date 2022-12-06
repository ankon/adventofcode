package _6

import (
	_ "embed"
	"fmt"
)

var knownInputs = map[string]int{
	"mjqjpqmgbljsphdztnvjfqwrcgsmlb": 7,
	// "bvwbjplbgvbhsrlpgdmjqwftvncz": 5,
	// "nppdvjthqldpwncqszvftbrmjlhg": 6,
	// "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg": 10,
	// "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw": 11,
}

//go:embed input.txt
var fullInput string

func Run(useSampleInput bool) error {
	for input, expectedResult := range knownInputs {
		result, err := detectMarker(input)
		if err != nil {
			return fmt.Errorf("tests failed: %w", err)
		}
		if result != expectedResult {
			return fmt.Errorf("unexpected result for %q: Expected %d, got %d", input, expectedResult, result)
		}
	}
	if useSampleInput {
		return nil
	}

	index, err := detectMarker(fullInput)
	if err != nil {
		return fmt.Errorf("failed to detect marker: %w", err)
	}
	fmt.Printf("Marker after %d\n", index)

	return nil
}

func isMarker(s string) bool {
	for i := 0; i < len(s); i++ {
		for j := 0; j < i; j++ {
			if s[j] == s[i] {
				return false
			}
		}
	}
	return true
}

func detectMarker(input string) (int, error) {
	for i := 4; i < len(input); i++ {
		if isMarker(input[i-4:i]) {
			return i, nil
		}
	}
	return -1, fmt.Errorf("cannot find marker")
}
