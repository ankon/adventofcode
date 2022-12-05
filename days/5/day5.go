package _5

import (
	_ "embed"
	"fmt"
	"math"
	"strings"

	"github.com/ankon/adventofcode/2022/days"
)

type stack struct {
	// Crates are named by single (upper-case) letters.
	crates []byte
}

func (s *stack) Top() byte {
	return s.crates[len(s.crates)-1]
}

func (s *stack) Pop() byte {
	end := len(s.crates)-1
	result := s.crates[end]
	s.crates = s.crates[:end]
	return result
}

func (s *stack) Push(x byte) {
	s.crates = append(s.crates, x)
}

type action struct {
	from int
	to   int
	count int
}

type parseState int
const (
	parsing_stacks parseState = 1
	parsing_actions parseState = 2
)

//go:embed sample.txt
var sampleInput string

//go:embed input.txt
var fullInput string

func simulateActions(stacks []stack, actions []action) error {
	for _, action := range actions {
		for i := 0; i < action.count; i++ {
			x := stacks[action.from - 1].Pop()
			stacks[action.to - 1].Push(x)
		}
	}
	return nil
}

func parseAction(l string) (action, error) {
	var count, from, to int

	n, err := fmt.Sscanf(l, "move %d from %d to %d", &count, &from, &to)
	if err != nil {
		return action{}, err
	}
	if n != 3 {
		return action{}, fmt.Errorf("unexpected number of fields %d, expected 3", n)
	}
	return action{from,to,count}, nil
}

func parseStacks(stackLines []string) ([]stack, error) {
	stackNames := stackLines[len(stackLines) - 1]
	stacks := make([]stack, int(math.Ceil(float64(len(stackNames)) / 4)))
	for i := len(stackLines) - 2; i >= 0; i-- {
		stackLine := stackLines[i]
		for s := 0; s < (len(stackLine) + 1) / 4; s++ {
			var crate byte
			n, err := fmt.Sscanf(stackLine[s*4:], "[%c]", &crate)
			if err == nil {
				if n != 1 {
					return nil, fmt.Errorf("expected 1 result, got %d", n)
				}
				stacks[s].Push(crate)
			}
		}
	}

	return stacks, nil
}

func parseInput(input string) (stacks []stack, actions []action, err error) {
	s := parsing_stacks
	stackLines := []string{}

	// Read lines until empty line
	// Line before empty line: Stack ids
	// Lines before that: stack contents
	// Read actions
	for _, line := range strings.Split(input, "\n") {
		if s == parsing_stacks {
			if line == "" {
				stacks, err = parseStacks(stackLines)
				if err != nil {
					return nil, nil, err
				}
				s = parsing_actions
			} else {
				stackLines = append(stackLines, line)
			}
		} else if s == parsing_actions {
			if line == "" {
				break
			}

			action, err := parseAction(line)
			if err != nil {
				return nil, nil, err
			}

			actions = append(actions, action)
		}
	}

	return stacks, actions, nil
}

func Run(useSampleInput bool) error {
	input := days.PickInput(useSampleInput, sampleInput, fullInput)
	stacks, actions, err := parseInput(input)
	if err != nil {
		return fmt.Errorf("cannot parse input: %w", err)
	}
	err = simulateActions(stacks, actions)
	if err != nil {
		return fmt.Errorf("cannot simulate actions: %w", err)
	}

	topCrates := ""
	for _, stack := range stacks {
		topCrates += string(stack.Top())
	}
	fmt.Printf("Top crates: %s\n", topCrates)

	return nil
}
