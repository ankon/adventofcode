package cmd

import (
	"fmt"
	"os"

	day1 "github.com/ankon/adventofcode/2022/days/1"
	day2 "github.com/ankon/adventofcode/2022/days/2"
	day3 "github.com/ankon/adventofcode/2022/days/3"
	day4 "github.com/ankon/adventofcode/2022/days/4"
	day5 "github.com/ankon/adventofcode/2022/days/5"
	day6 "github.com/ankon/adventofcode/2022/days/6"
	day7 "github.com/ankon/adventofcode/2022/days/7"

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
	{"Tuning Trouble", day6.Run},
	{"No Space Left On Device", day7.Run},
}

func init() {
	for index, day := range days {
		cmd := &cobra.Command{
			Use:   fmt.Sprintf("day%d", index + 1),
			Short: day.short,
			Run: func(cmd *cobra.Command, args []string) {
				err := day.run(useSampleInput)
				if err != nil {
					fmt.Printf("Error: %v", err)
					os.Exit(1)
				}
			},
		}
		rootCmd.AddCommand(cmd)
	}
}
