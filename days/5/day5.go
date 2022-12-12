package _5

import (
	_ "embed"
	"fmt"
	"math"
	"strings"

	"github.com/ankon/adventofcode/2022/days"
	"github.com/ankon/adventofcode/2022/utils"
)

type crateStack = utils.Stack[byte]

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

func simulateActionsCrateMover9000(stacks []crateStack, actions []action) error {
	for _, action := range actions {
		for i := 0; i < action.count; i++ {
			x := stacks[action.from - 1].Pop()
			stacks[action.to - 1].Push(x)
		}
	}
	return nil
}

func simulateActionsCrateMover9001(stacks []crateStack, actions []action) error {
	for _, action := range actions {
		if action.count == 1 {
			x := stacks[action.from - 1].Pop()
			stacks[action.to - 1].Push(x)
		} else {
			helper := crateStack{}
			for i := 0; i < action.count; i++ {
				x := stacks[action.from - 1].Pop()
				helper.Push(x)
			}
			for i := 0; i < action.count; i++ {
				x := helper.Pop()
				stacks[action.to - 1].Push(x)
			}
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

func parseStacks(stackLines []string) ([]crateStack, error) {
	stackNames := stackLines[len(stackLines) - 1]
	stacks := make([]crateStack, int(math.Ceil(float64(len(stackNames)) / 4)))
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

func parseInput(input string) (stacks []crateStack, actions []action, err error) {
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

func getTopCrates(stacks []crateStack) string {
	result := ""
	for _, stack := range stacks {
		result += string(stack.Top())
	}
	return result
}

func Run(useSampleInput bool) error {
	input := days.PickInput(useSampleInput, sampleInput, fullInput)
	inputStacks, actions, err := parseInput(input)
	if err != nil {
		return fmt.Errorf("cannot parse input: %w", err)
	}

	stacks := make([]crateStack, len(inputStacks))
	for i, stack := range inputStacks {
		stacks[i] = stack.Clone()
	}
	err = simulateActionsCrateMover9000(stacks, actions)
	if err != nil {
		return fmt.Errorf("cannot simulate actions: %w", err)
	}
	fmt.Printf("Top crates (CrateMover 9000): %s\n", getTopCrates(stacks))

	for i, stack := range inputStacks {
		stacks[i] = stack.Clone()
	}
	err = simulateActionsCrateMover9001(stacks, actions)
	if err != nil {
		return fmt.Errorf("cannot simulate actions: %w", err)
	}
	fmt.Printf("Top crates (CrateMover 9001): %s\n", getTopCrates(stacks))

	return nil
}
