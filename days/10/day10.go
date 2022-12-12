package _10

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"

	"github.com/ankon/adventofcode/2022/days"
	"github.com/spf13/cobra"
)

var trivialInput = `
	noop
	addx 3
	addx -5`

//go:embed sample.txt
var sampleInput string

//go:embed input.txt
var fullInput string

var debug = false

func ConfigureCommand(cmd *cobra.Command) {
	cmd.Flags().BoolVar(&debug, "debug", false, "Enable debug output")
}

func Run(useSampleInput bool) error {
	input := days.PickInput(useSampleInput, sampleInput, fullInput)
	instructions := strings.Split(strings.TrimSpace(input), "\n")

	sum := 0
	sampleTask1 := func(cycle int, state state) {
		if debug {
			fmt.Printf("%d: X=%d\n", cycle, state.x)
		}
		if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
			sum += cycle * state.x
		}
	}
	err := simulateProgram(instructions, state{1}, sampleTask1)
	if err != nil {
		return err
	}
	fmt.Printf("Signal strength sum %d\n", sum)

	lines := []string{}
	line := ""
	pos := 0
	crt := func(cycle int, state state) {
		if debug {
			fmt.Printf("%d: Sprite at %d, drawing pos %d\n", cycle, state.x, pos)
			fmt.Printf("%d: %s\n", cycle, line)
		}

		if state.x-1 <= pos && state.x+1 >= pos {
			line += "#"
		} else {
			line += "."
		}
		pos++

		if cycle%40 == 0 {
			lines = append(lines, line)
			line = ""
			pos = 0
		}
	}
	err = simulateProgram(instructions, state{1}, crt)
	if err != nil {
		return err
	}
	fmt.Printf("CRT %d\n", sum)
	for _, line := range lines {
		fmt.Println(line)
	}

	return nil
}

type state struct {
	x int
}

func simulateProgram(instructions []string, initialState state, sample func(cycle int, state state)) error {
	cycle := 1
	state := initialState

	tick := func() {
		sample(cycle, state)
		cycle++
	}

	for _, ins := range instructions {
		op, arg, _ := strings.Cut(ins, " ")

		switch op {
		case "noop":
			tick()
		case "addx":
			tick()

			tick()
			v, err := strconv.Atoi(arg)
			if err != nil {
				return fmt.Errorf("invalid arg %q to addx at %d", arg, cycle)
			}
			state.x += v
		default:
			return fmt.Errorf("invalid op %q at %d", op, cycle)
		}
	}

	// One more tick, to sample the end state
	tick()

	return nil
}
