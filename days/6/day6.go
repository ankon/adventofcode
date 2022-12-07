package _6

import (
	_ "embed"
	"fmt"
)

type expected struct {
	packet int
	message int
}

type requiredLength int
const (
	startOfPacket requiredLength  = 4
	startOfMessage requiredLength = 14
)

var knownInputs = map[string]expected{
	"mjqjpqmgbljsphdztnvjfqwrcgsmlb": {7, 19},
	"bvwbjplbgvbhsrlpgdmjqwftvncz": {5, 23},
	"nppdvjthqldpwncqszvftbrmjlhg": {6, 23},
	"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg": {10, 29},
	"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw": {11, 26},
}

//go:embed input.txt
var fullInput string

func Run(useSampleInput bool) error {
	for input, expectedResult := range knownInputs {
		result, err := detectMarker(input, startOfPacket)
		if err != nil {
			return fmt.Errorf("tests failed: %w", err)
		}
		if result != expectedResult.packet {
			return fmt.Errorf("unexpected start-of-packet marker for %q: Expected %d, got %d", input, expectedResult.packet, result)
		}
		result, err = detectMarker(input, startOfMessage)
		if err != nil {
			return fmt.Errorf("tests failed: %w", err)
		}
		if result != expectedResult.message {
			return fmt.Errorf("unexpected start-of-message marker for %q: Expected %d, got %d", input, expectedResult.message, result)
		}
	}
	if useSampleInput {
		return nil
	}

	index, err := detectMarker(fullInput, startOfPacket)
	if err != nil {
		return fmt.Errorf("failed to detect start-of-packet marker: %w", err)
	}
	fmt.Printf("start-of-packet marker after %d\n", index)

	index, err = detectMarker(fullInput, startOfMessage)
	if err != nil {
		return fmt.Errorf("failed to detect start-of-message marker: %w", err)
	}
	fmt.Printf("start-of-message marker after %d\n", index)

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

func detectMarker(input string, t requiredLength) (int, error) {
	for i := int(t); i < len(input); i++ {
		if isMarker(input[i-int(t):i]) {
			return i, nil
		}
	}
	return -1, fmt.Errorf("cannot find marker")
}
