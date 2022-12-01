package _1

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"
)

//go:embed sample.txt
var sampleInput string

//go:embed input.txt
var fullInput string

func findHighestCaloriesOnElf(input string) (int, error) {
    var highest int
    var current int
    for _, line := range strings.Split(input, "\n") {
        if line == "" {
            if current > highest {
                highest = current
            }
            current = 0
        } else {
            calories, err := strconv.ParseInt(line, 0, 0)
            if err != nil {
                return 0, err
            }
            current += int(calories)
        }
    }
    return highest, nil
}

func pickInput(useSampleInput bool) string {
    if useSampleInput {
        return sampleInput
    } else {
        return fullInput
    }
}

func Run(useSampleInput bool) {
    input := pickInput(useSampleInput)
    highest, err := findHighestCaloriesOnElf(input)
    if err != nil {
        panic(err)
    }

    fmt.Printf("Highest calorie count on an elf: %d\n", highest)
}
