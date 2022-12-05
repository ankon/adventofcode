package cmd

import (
	"fmt"

	day1 "github.com/ankon/adventofcode/2022/days/1"
	day2 "github.com/ankon/adventofcode/2022/days/2"
	day3 "github.com/ankon/adventofcode/2022/days/3"
	day4 "github.com/ankon/adventofcode/2022/days/4"
	day5 "github.com/ankon/adventofcode/2022/days/5"

	"github.com/spf13/cobra"
)

type dayRunFunc func(bool) error
type day struct {
	short string
	run dayRunFunc
}

var days = []day{
	{"Calorie Counting", day1.Run},
	{"Rock Paper Scissors", day2.Run},
	{"Rucksack Reorganization", day3.Run},
	{"Camp Cleanup", day4.Run},
	{"Supply Stacks", day5.Run},
}

func init() {
	for index, day := range days {
		cmd := &cobra.Command{
			Use:   fmt.Sprintf("day%d", index + 1),
			Short: day.short,
			Run: func(cmd *cobra.Command, args []string) {
				day.run(useSampleInput)
			},
		}
		rootCmd.AddCommand(cmd)
	}
}
