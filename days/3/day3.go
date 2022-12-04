package _3

import (
	_ "embed"
	"fmt"
	"strings"
	"unicode"
)

func findCommonItem(compartment1, compartment2 string) (rune, error) {
	for _, c := range compartment2 {
		if strings.ContainsRune(compartment1, c) {
			return c, nil
		}
	}
	return -1, fmt.Errorf("no common item")
}

func calculatePrioritySum(rucksacks []string) (int, error) {
	var sum int
	for _, rucksack := range rucksacks {
		if rucksack == "" {
			continue
		}
		l := len(rucksack)
		compartment1, compartment2 := rucksack[0:l/2], rucksack[l/2:]
		commonItem, err := findCommonItem(compartment1, compartment2)
		if err != nil {
			return 0, fmt.Errorf("cannot find common item for %q: %w", rucksack, err)
		}
		if unicode.IsUpper(commonItem) {
			sum += int(commonItem - 'A') + 27
		} else {
			sum += int(commonItem - 'a') + 1
		}
	}
	return sum, nil
}

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

func Run(useSampleInput bool) {
	input := pickInput(useSampleInput)
	rucksacks := strings.Split(input, "\n")
	prioritySum, err := calculatePrioritySum(rucksacks)
	if err != nil {
		panic(err)
	}
	fmt.Printf("Sum of priorities: %d\n", prioritySum)
}
