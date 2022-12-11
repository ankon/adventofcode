package cmd

import (
	"fmt"
	"os"

	day1 "github.com/ankon/adventofcode/2022/days/1"
	day10 "github.com/ankon/adventofcode/2022/days/10"
	day2 "github.com/ankon/adventofcode/2022/days/2"
	day3 "github.com/ankon/adventofcode/2022/days/3"
	day4 "github.com/ankon/adventofcode/2022/days/4"
	day5 "github.com/ankon/adventofcode/2022/days/5"
	day6 "github.com/ankon/adventofcode/2022/days/6"
	day7 "github.com/ankon/adventofcode/2022/days/7"
	day8 "github.com/ankon/adventofcode/2022/days/8"
	day9 "github.com/ankon/adventofcode/2022/days/9"

	"github.com/spf13/cobra"
)

type dayRunFunc func(bool) error
type configureCommandFunc func(*cobra.Command)

var Default configureCommandFunc = func(c *cobra.Command) {}

type day struct {
	short string
	run dayRunFunc
	configureCommand configureCommandFunc
}

var days = []day{
	{"Calorie Counting", day1.Run, Default},
	{short: "Rock Paper Scissors", run: day2.Run, configureCommand: Default},
	{"Rucksack Reorganization", day3.Run, Default},
	{"Camp Cleanup", day4.Run, Default},
	{"Supply Stacks", day5.Run, Default},
	{"Tuning Trouble", day6.Run, Default},
	{"No Space Left On Device", day7.Run, Default},
	{"Treetop Tree House", day8.Run, Default},
	{"Rope Bridge", day9.Run, day9.ConfigureCommand},
	{"Cathode-Ray Tube", day10.Run, day10.ConfigureCommand},
}

func init() {
	for index, day := range days {
		run := day.run
		cmd := &cobra.Command{
			Use:   fmt.Sprintf("day%d", index + 1),
			Short: day.short,
			Run: func(cmd *cobra.Command, args []string) {
				err := run(useSampleInput)
				if err != nil {
					fmt.Printf("Error: %v", err)
					os.Exit(1)
				}
			},
		}
		day.configureCommand(cmd)
		rootCmd.AddCommand(cmd)
	}
}
