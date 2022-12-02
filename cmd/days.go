package cmd

import (
	day1 "github.com/ankon/adventofcode/2022/days/1"
	day2 "github.com/ankon/adventofcode/2022/days/2"

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
	Short: "Rock Paper Scissor",
	Run: func(cmd *cobra.Command, args []string) {
		day2.Run(useSampleInput)
	},
}

func init() {
	rootCmd.AddCommand(day1Cmd)
	rootCmd.AddCommand(day2Cmd)
}
