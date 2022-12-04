package cmd

import (
	day1 "github.com/ankon/adventofcode/2022/days/1"
	day2 "github.com/ankon/adventofcode/2022/days/2"
	day3 "github.com/ankon/adventofcode/2022/days/3"

	"github.com/spf13/cobra"
)

var day1Cmd = &cobra.Command{
	Use:   "day1",
	Short: "Calorie Counting",
	Run: func(cmd *cobra.Command, args []string) {
		day1.Run(useSampleInput)
	},
}

var day2Cmd = &cobra.Command{
	Use:   "day2",
	Short: "Rock Paper Scissors",
	Run: func(cmd *cobra.Command, args []string) {
		day2.Run(useSampleInput)
	},
}

var day3Cmd = &cobra.Command{
	Use:   "day3",
	Short: "Rucksack Reorganization",
	Run: func(cmd *cobra.Command, args []string) {
		day3.Run(useSampleInput)
	},
}

func init() {
	rootCmd.AddCommand(day1Cmd)
	rootCmd.AddCommand(day2Cmd)
	rootCmd.AddCommand(day3Cmd)
}
