package _2

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

type shape int

const (
	Rock     shape = 1
	Paper    shape = 2
	Scissors shape = 3
)

type outcome int

const (
	Lose outcome = 0
	Draw outcome = 3
	Win  outcome = 6
)

var roundValueSimpleTheory = map[string]int{
	"A X": int(Draw) + int(Rock),
	"A Y": int(Win) + int(Paper),
	"A Z": int(Lose) + int(Scissors),
	"B X": int(Lose) + int(Rock),
	"B Y": int(Draw) + int(Paper),
	"B Z": int(Win) + int(Scissors),
	"C X": int(Win) + int(Rock),
	"C Y": int(Lose) + int(Paper),
	"C Z": int(Draw) + int(Scissors),
}

var roundValueActualMeaning = map[string]int{
	"A X": int(Lose) + int(Scissors),
	"A Y": int(Draw) + int(Rock),
	"A Z": int(Win) + int(Paper),
	"B X": int(Lose) + int(Rock),
	"B Y": int(Draw) + int(Paper),
	"B Z": int(Win) + int(Scissors),
	"C X": int(Lose) + int(Paper),
	"C Y": int(Draw) + int(Scissors),
	"C Z": int(Win) + int(Rock),
}

func simulateGame(rounds []string, roundValue map[string]int) (int, error) {
	var score int
	for _, round := range rounds {
		if round == "" {
			continue
		}

		score += roundValue[round]
	}
	return score, nil
}

func Run(useSampleInput bool) error {
	input := days.PickInput(useSampleInput, sampleInput, fullInput)

	rounds := strings.Split(input, "\n")

	assumedScore, err := simulateGame(rounds, roundValueSimpleTheory)
	if err != nil {
		return err
	}
	fmt.Printf("Score according to theory about guide: %d\n", assumedScore)

	actualScore, err := simulateGame(rounds, roundValueActualMeaning)
	if err != nil {
		return err
	}
	fmt.Printf("Score according to actual meaning of guide: %d\n", actualScore)

	return nil
}
