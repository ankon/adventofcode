package _2

import (
	_ "embed"
	"fmt"
	"strings"
)

//go:embed sample.txt
var sampleInput string

//go:embed input.txt
var fullInput string

func pickInput(useSampleInput bool) string {
    if useSampleInput {
        return sampleInput
    } else {
        return fullInput
    }
}

var roundValue = map[string]int{
    "A X": 3 + 1,
    "A Y": 6 + 2,
    "A Z": 0 + 3,
    "B X": 0 + 1,
    "B Y": 3 + 2,
    "B Z": 6 + 3,
    "C X": 6 + 1,
    "C Y": 0 + 2,
    "C Z": 3 + 3,
}

func simulateGame(rounds []string) (int, error) {
    var score int
    for _, round := range rounds {
        if round == "" {
            continue
        }

        score += roundValue[round]
    }
    return score, nil
}

func Run(useSampleInput bool) {
    input := pickInput(useSampleInput)

    rounds := strings.Split(input, "\n")
    score, err := simulateGame(rounds)
    if err != nil {
        panic(err)
    }
    fmt.Printf("Score according to guide: %d\n", score)
}
