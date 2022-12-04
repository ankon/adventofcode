package _3

import (
	_ "embed"
	"fmt"
	"strings"

	"github.com/ankon/adventofcode/2022/days"
	"golang.org/x/exp/slices"
)

func findCommonItem(compartment1, compartment2 []byte) (byte, error) {
	for _, c := range compartment2 {
		if slices.Contains(compartment1, c) {
			return c, nil
		}
	}
	return 0, fmt.Errorf("no common item")
}

func getPriority(item byte) int {
	if item >= 'A' && item <= 'Z' {
		return int(item - 'A') + 27
	} else {
		return int(item - 'a') + 1
	}
}

func calculateMispackedItemInCompartmentsPrioritySum(rucksacks []string) (int, error) {
	sum := 0
	for _, rucksack := range rucksacks {
		if len(rucksack) == 0 {
			continue
		}

		l := len(rucksack)
		compartment1, compartment2 := rucksack[0:l/2], rucksack[l/2:]
		commonItem, err := findCommonItem([]byte(compartment1), []byte(compartment2))
		if err != nil {
			return 0, fmt.Errorf("cannot find common item for %q: %w", rucksack, err)
		}
		sum += getPriority(commonItem)
	}
	return sum, nil
}

func commonItemTypes(list1, list2 []byte) []byte {
	result := []byte{}
	for _, c := range list2 {
		if slices.Contains(result, c) {
			continue
		}
		if slices.Contains(list1, c) {
			result = append(result, c)
		}
	}
	return result
}

func calculateGroupBadgeItemPrioritySum(rucksacks []string) (int, error) {
	sum := 0
	for i := 0; i + 3 < len(rucksacks); i += 3 {
		common1 := commonItemTypes([]byte(rucksacks[i]), []byte(rucksacks[i+1]))
		common := commonItemTypes(common1, []byte(rucksacks[i+2]))
		if len(common) != 1 {
			return 0, fmt.Errorf("expected only one common item, but got %q", common)
		}
		sum += getPriority(common[0])
	}
	return sum, nil
}

//go:embed sample.txt
var sampleInput string

//go:embed input.txt
var fullInput string

func Run(useSampleInput bool) error {
	input := days.PickInput(useSampleInput, sampleInput, fullInput)

	rucksacks := strings.Split(input, "\n")
	prioritySum, err := calculateMispackedItemInCompartmentsPrioritySum(rucksacks)
	if err != nil {
		return err
	}
	fmt.Printf("Sum of priorities: %d\n", prioritySum)

	groupBadgePrioritySym, err := calculateGroupBadgeItemPrioritySum(rucksacks)
	if err != nil {
		return err
	}
	fmt.Printf("Group badge priority sum: %d\n", groupBadgePrioritySym)

	return nil
}
